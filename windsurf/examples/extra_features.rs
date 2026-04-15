use core::cell::RefCell;
use windsurf::{
    CursorMode, CursorSource, DragSource, FeatureSet, Features, ImeState, UnsupportedFeature,
    WindowId,
};

#[derive(Default)]
struct MockExtrasBackend {
    ime_enabled: RefCell<bool>,
    cursor_locked: RefCell<bool>,
}

impl Features for MockExtrasBackend {
    fn supported_features(&self) -> FeatureSet {
        FeatureSet::IME.with(FeatureSet::CURSOR)
    }

    fn set_ime_state(&self, _window: WindowId, state: &ImeState) -> Result<(), UnsupportedFeature> {
        *self.ime_enabled.borrow_mut() = state.enabled;
        Ok(())
    }

    fn set_cursor(
        &self,
        _window: WindowId,
        _source: &CursorSource,
    ) -> Result<(), UnsupportedFeature> {
        Ok(())
    }

    fn set_cursor_mode(
        &self,
        _window: WindowId,
        mode: CursorMode,
    ) -> Result<(), UnsupportedFeature> {
        *self.cursor_locked.borrow_mut() = matches!(mode, CursorMode::Locked);
        Ok(())
    }

    fn start_drag(&self, _window: WindowId, _source: DragSource) -> Result<(), UnsupportedFeature> {
        Err(UnsupportedFeature::new(
            windsurf::FeatureKind::DragDropSource,
        ))
    }
}

fn main() {
    let backend = MockExtrasBackend::default();
    let window = WindowId::new(7);

    backend
        .set_ime_state(
            window,
            &ImeState {
                enabled: true,
                ..ImeState::default()
            },
        )
        .expect("mock backend supports IME");

    backend
        .set_cursor_mode(window, CursorMode::Locked)
        .expect("mock backend supports cursor mode changes");

    assert!(backend.supported_features().contains(FeatureSet::IME));
    assert!(*backend.ime_enabled.borrow());
    assert!(*backend.cursor_locked.borrow());
}
