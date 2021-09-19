use std::borrow::Cow;
use std::collections::HashMap;

use html5ever::tendril::*;
#[allow(unused)]
use html5ever::tree_builder::{
    AppendNode, AppendText, ElementFlags, NodeOrText, QuirksMode, TreeSink,
};
use html5ever::{Attribute, ExpandedName, QualName};

use std::str;

pub struct Sink {
    pub next_id: usize,
    pub names: HashMap<usize, QualName>,
    pub a_href: Vec<String>,
    pub a_name: Vec<String>,
}

impl Sink {
    pub fn get_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 2;
        id
    }
}

impl TreeSink for Sink {
    type Handle = usize;
    type Output = Self;
    fn finish(self) -> Self {
        self
    }

    #[allow(unused_variables)]
    fn parse_error(&mut self, msg: Cow<'static, str>) {
        // println!("Parse error: {}", msg);
    }

    fn get_document(&mut self) -> usize {
        0
    }

    fn get_template_contents(&mut self, target: &usize) -> usize {
        if let Some(expanded_name!(html "template")) = self.names.get(target).map(|n| n.expanded())
        {
            target + 1
        } else {
            panic!("not a template element")
        }
    }

    #[allow(unused_variables)]
    fn set_quirks_mode(&mut self, mode: QuirksMode) {
        // println!("Set quirks mode to {:?}", mode);
    }

    fn same_node(&self, x: &usize, y: &usize) -> bool {
        x == y
    }

    fn elem_name(&self, target: &usize) -> ExpandedName {
        self.names.get(target).expect("not an element").expanded()
    }

    fn create_element(&mut self, name: QualName, attrs: Vec<Attribute>, _: ElementFlags) -> usize {
        let id = self.get_id();
        // println!("Created {:?} as {}", name, id);
        if str::from_utf8(name.local.as_bytes()).unwrap() == "a" {
            for attr in &attrs {
                match str::from_utf8(attr.name.local.as_bytes()) {
                    Ok("href") => {
                        self.a_href
                            .push(str::from_utf8(attr.value.as_bytes()).unwrap().to_string());
                    }
                    Ok("name") => {
                        self.a_name.push(
                            str::from_utf8(&attr.value.as_bytes()[1..])
                                .unwrap()
                                .to_string(),
                        );
                    }
                    _ => {}
                }
                // println!("[attr]    {:?} = {}", attr.name, attr.value);
            }
        }

        self.names.insert(id, name);
        id
    }

    #[allow(unused_variables)]
    fn create_comment(&mut self, text: StrTendril) -> usize {
        let id = self.get_id();
        // println!("Created comment \"{}\" as {}", text.escape_default(), id);
        id
    }

    #[allow(unused_variables)]
    fn create_pi(&mut self, target: StrTendril, value: StrTendril) -> usize {
        unimplemented!()
    }

    #[allow(unused_variables)]
    fn append(&mut self, parent: &usize, child: NodeOrText<usize>) {
        // match child {
        //     AppendNode(n) => println!("Append node {} to {}", n, parent),
        //     AppendText(t) => println!("Append text to {}: \"{}\"", parent, t.escape_default()),
        // }
    }

    #[allow(unused_variables)]
    fn append_before_sibling(&mut self, sibling: &usize, new_node: NodeOrText<usize>) {
        // match new_node {
        //     AppendNode(n) => println!("Append node {} before {}", n, sibling),
        //     AppendText(t) => println!("Append text before {}: \"{}\"", sibling, t.escape_default()),
        // }
    }

    fn append_based_on_parent_node(
        &mut self,
        element: &Self::Handle,
        _prev_element: &Self::Handle,
        child: NodeOrText<Self::Handle>,
    ) {
        self.append_before_sibling(element, child);
    }

    #[allow(unused_variables)]
    fn append_doctype_to_document(
        &mut self,
        name: StrTendril,
        public_id: StrTendril,
        system_id: StrTendril,
    ) {
        // println!("Append doctype: {} {} {}", name, public_id, system_id);
    }

    #[allow(unused_variables)]
    fn add_attrs_if_missing(&mut self, target: &usize, attrs: Vec<Attribute>) {
        assert!(self.names.contains_key(target), "not an element");
        // println!("Add missing attributes to {}:", target);
        // for attr in attrs.into_iter() {
        //     println!("    {:?} = {}", attr.name, attr.value);
        // }
    }

    fn associate_with_form(
        &mut self,
        _target: &usize,
        _form: &usize,
        _nodes: (&usize, Option<&usize>),
    ) {
        // No form owner support.
    }

    #[allow(unused_variables)]
    fn remove_from_parent(&mut self, target: &usize) {
        // println!("Remove {} from parent", target);
    }

    #[allow(unused_variables)]
    fn reparent_children(&mut self, node: &usize, new_parent: &usize) {
        // println!("Move children from {} to {}", node, new_parent);
    }

    #[allow(unused_variables)]
    fn mark_script_already_started(&mut self, node: &usize) {
        // println!("Mark script {} as already started", node);
    }

    #[allow(unused_variables)]
    fn set_current_line(&mut self, line_number: u64) {
        // println!("Set current line to {}", line_number);
    }

    #[allow(unused_variables)]
    fn pop(&mut self, elem: &usize) {
        // println!("Popped element {}", elem);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample_test() {}
}
