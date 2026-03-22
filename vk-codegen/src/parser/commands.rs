//! Parser for Vulkan commands.

use crate::ir::{
    CType, CmdBufferLevel, Command, ExportScope, QueueType, Registry, RenderPass, TaskType,
    parse_dep_expr,
};
use crate::parser::nodes::{
    api_set, attr, child_name, depr_info, parse_c_type, parse_member, true_false_panic,
    true_or_panic,
};
use roxmltree::Node;

/// Parses a commands block from an XML node into the Registry.
///
/// Arguments:
///
/// - `node`: The XML node containing the commands block.
/// - `reg`: The mutable Registry instance to populate.
pub fn parse_commands_block(node: Node, reg: &mut Registry) {
    for cn in node
        .children()
        .filter(|n| n.is_element() && n.tag_name().name() == "command")
    {
        let alias = attr(cn, "alias").map(str::to_owned);
        let name_attr = attr(cn, "name").map(str::to_owned);
        let aset = api_set(cn);
        let dep = attr(cn, "depends").map(parse_dep_expr);
        let depr = depr_info(cn);
        let comment = attr(cn, "comment").map(str::to_owned);

        if alias.is_some() {
            if let Some(name) = name_attr {
                reg.commands.entry(name.clone()).or_default().push(Command {
                    name,
                    alias,
                    return_type: CType::simple("VkResult"),
                    params: vec![],
                    api: aset,
                    comment,
                    dep,
                    provided_by: vec![],
                    depr,
                    success_codes: vec![],
                    error_codes: vec![],
                    conditional_rendering: false,
                    render_pass: None,
                    cmd_buffer_levels: vec![],
                    tasks: vec![],
                    extern_sync: None,
                    allow_no_queues: false,
                    queues: vec![],
                    export: attr(cn, "export")
                        .map(ExportScope::parse)
                        .unwrap_or_default(),
                });
            }
            continue;
        }

        let proto = cn
            .children()
            .find(|n| n.is_element() && n.tag_name().name() == "proto");
        let (name, return_type) = if let Some(proto) = proto {
            (child_name(proto).unwrap_or_default(), parse_c_type(proto))
        } else {
            (name_attr.unwrap_or_default(), CType::simple("void"))
        };
        if name.is_empty() {
            continue;
        }

        let mut params: Vec<crate::ir::Member> = Vec::new();
        for pn in cn
            .children()
            .filter(|n| n.is_element() && n.tag_name().name() == "param")
        {
            let p = parse_member(pn);
            if let Some(existing) = params.iter_mut().find(|m| m.name == p.name) {
                if let (Some(a1), Some(a2)) = (&mut existing.api, &p.api) {
                    a1.vulkan |= a2.vulkan;
                    a1.vulkansc |= a2.vulkansc;
                    a1.vulkanbase |= a2.vulkanbase;
                }
            } else {
                params.push(p);
            }
        }
        let render_pass = attr(cn, "renderpass").map(RenderPass::parse);
        reg.commands.entry(name.clone()).or_default().push(Command {
            name,
            alias,
            return_type,
            params,
            api: aset,
            comment,
            dep,
            provided_by: vec![],
            depr,
            success_codes: attr(cn, "successcodes")
                .map(|s| s.split(',').map(str::to_owned).collect())
                .unwrap_or_default(),
            error_codes: attr(cn, "errorcodes")
                .map(|s| s.split(',').map(str::to_owned).collect())
                .unwrap_or_default(),
            conditional_rendering: attr(cn, "conditionalrendering").is_some_and(true_false_panic),
            render_pass,
            cmd_buffer_levels: attr(cn, "cmdbufferlevels")
                .map(CmdBufferLevel::parse)
                .unwrap_or_default(),
            tasks: attr(cn, "tasks").map(TaskType::parse).unwrap_or_default(),
            extern_sync: attr(cn, "externsync").map(str::to_owned),
            allow_no_queues: attr(cn, "allownoqueues").is_some_and(true_or_panic),
            queues: attr(cn, "queues").map(QueueType::parse).unwrap_or_default(),
            export: attr(cn, "export")
                .map(ExportScope::parse)
                .unwrap_or_default(),
        });
    }
}
