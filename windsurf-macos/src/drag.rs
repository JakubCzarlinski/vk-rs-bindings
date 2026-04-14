use crate::state::{SharedDisplayRef, SharedState};
use alloc::rc::{Rc, Weak};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::cell::RefCell;
use objc2::rc::Retained;
use objc2::runtime::{AnyClass, ProtocolObject};
use objc2::{ClassType, DefinedClass, MainThreadMarker, MainThreadOnly, define_class, msg_send};
use objc2_app_kit::{
    NSDragOperation, NSDraggingDestination, NSDraggingInfo, NSPasteboard, NSPasteboardTypeFileURL,
    NSWindow, NSWindowDelegate,
};
use objc2_foundation::{NSArray, NSObject, NSObjectProtocol, NSPoint, NSURL};
use std::path::PathBuf;
use windsurf_core::{LogicalPosition, WindowId};
use windsurf_extra::{DragAction, DragData, DragDropEvent, ExtraEvent};

extern crate alloc;
extern crate std;

pub(crate) struct DragDelegateIvars {
    shared: Weak<RefCell<SharedState>>,
    window_id: WindowId,
}

define_class!(
    // SAFETY:
    // - `NSObject` has no subclassing requirements.
    // - `WindowDragDelegate` does not implement `Drop`.
    #[unsafe(super = NSObject)]
    #[thread_kind = MainThreadOnly]
    #[ivars = DragDelegateIvars]
    pub(crate) struct WindowDragDelegate;

    unsafe impl NSObjectProtocol for WindowDragDelegate {}
    unsafe impl NSWindowDelegate for WindowDragDelegate {}

    unsafe impl NSDraggingDestination for WindowDragDelegate {
        #[unsafe(method(draggingEntered:))]
        fn dragging_entered(&self, sender: &ProtocolObject<dyn NSDraggingInfo>) -> NSDragOperation {
            let window_id = self.ivars().window_id;
            let pasteboard = sender.draggingPasteboard();
            let offered = offered_drag_types(&pasteboard);
            let position = self.drag_position(sender.draggingLocation());

            if self.push_drag_event(DragDropEvent::Entered {
                id: window_id,
                position,
                offered,
            }) {
                NSDragOperation::Copy
            } else {
                NSDragOperation::None
            }
        }

        #[unsafe(method(draggingUpdated:))]
        fn dragging_updated(&self, sender: &ProtocolObject<dyn NSDraggingInfo>) -> NSDragOperation {
            let window_id = self.ivars().window_id;
            let position = self.drag_position(sender.draggingLocation());
            if self.push_drag_event(DragDropEvent::Moved {
                id: window_id,
                position,
            }) {
                NSDragOperation::Copy
            } else {
                NSDragOperation::None
            }
        }

        #[unsafe(method(draggingExited:))]
        fn dragging_exited(&self, _sender: Option<&ProtocolObject<dyn NSDraggingInfo>>) {
            let window_id = self.ivars().window_id;
            let _ = self.push_drag_event(DragDropEvent::Left { id: window_id });
        }

        #[unsafe(method(prepareForDragOperation:))]
        fn prepare_for_drag_operation(&self, _sender: &ProtocolObject<dyn NSDraggingInfo>) -> bool {
            true
        }

        #[unsafe(method(performDragOperation:))]
        fn perform_drag_operation(&self, sender: &ProtocolObject<dyn NSDraggingInfo>) -> bool {
            let window_id = self.ivars().window_id;
            let position = self.drag_position(sender.draggingLocation());
            let files = read_dragged_files(&sender.draggingPasteboard());
            if files.is_empty() {
                return false.into();
            }

            self.push_drag_event(DragDropEvent::Dropped {
                id: window_id,
                position,
                data: Vec::from([DragData::Files(files)]),
                action: DragAction::Copy,
            })
        }
    }
);

impl WindowDragDelegate {
    fn new(
        mtm: MainThreadMarker,
        shared: &SharedDisplayRef,
        window_id: WindowId,
    ) -> Retained<Self> {
        let this = Self::alloc(mtm).set_ivars(DragDelegateIvars {
            shared: Rc::downgrade(shared),
            window_id,
        });
        // SAFETY: Calling `init` on `NSObject` subclasses is safe.
        unsafe { msg_send![super(this), init] }
    }

    fn drag_position(&self, point: NSPoint) -> LogicalPosition {
        let window_id = self.ivars().window_id;
        if let Some(shared) = self.ivars().shared.upgrade() {
            let shared = shared.borrow();
            if let Some(state) = shared.windows.get(&window_id) {
                return LogicalPosition::new(point.x, f64::from(state.size.height) - point.y);
            }
        }
        LogicalPosition::new(point.x, point.y)
    }

    fn push_drag_event(&self, event: DragDropEvent) -> bool {
        let window_id = self.ivars().window_id;
        let Some(shared) = self.ivars().shared.upgrade() else {
            return false;
        };
        let mut shared = shared.borrow_mut();
        if !shared.windows.contains_key(&window_id) {
            return false;
        }
        shared.push_extra(ExtraEvent::DragDrop(event));
        true
    }
}

pub(crate) fn attach_file_drop_delegate(
    mtm: MainThreadMarker,
    shared: &SharedDisplayRef,
    window: &NSWindow,
    window_id: WindowId,
) -> Retained<WindowDragDelegate> {
    let drag_delegate = WindowDragDelegate::new(mtm, shared, window_id);
    window.setDelegate(Some(ProtocolObject::from_ref(&*drag_delegate)));
    let drag_types = NSArray::from_slice(&[unsafe { NSPasteboardTypeFileURL }]);
    window.registerForDraggedTypes(&drag_types);
    drag_delegate
}

fn offered_drag_types(pasteboard: &NSPasteboard) -> Vec<String> {
    let Some(types) = pasteboard.types() else {
        return Vec::new();
    };

    let count = types.count();
    let mut offered = Vec::with_capacity(count);
    for index in 0..count {
        offered.push(types.objectAtIndex(index).to_string());
    }
    offered
}

fn read_dragged_files(pasteboard: &NSPasteboard) -> Vec<PathBuf> {
    let classes = NSArray::from_slice(&[NSURL::class() as &'static AnyClass]);
    // SAFETY: `NSURL` is a valid `NSPasteboardReading` class for file URL drops.
    let Some(objects) = (unsafe { pasteboard.readObjectsForClasses_options(&classes, None) })
    else {
        return Vec::new();
    };

    let mut files = Vec::new();
    let count = objects.count();
    for index in 0..count {
        let object = objects.objectAtIndex(index);
        let Ok(url) = object.downcast::<NSURL>() else {
            continue;
        };
        if !url.isFileURL() {
            continue;
        }
        let Some(path) = url.path() else {
            continue;
        };
        files.push(PathBuf::from(path.to_string()));
    }

    files
}
