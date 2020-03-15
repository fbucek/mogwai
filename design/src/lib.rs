
// Give GizmoBuilder a tyvar:

//struct GizmoBuilder<T>;


// Then define different options:

pub enum ElementOptions {
  
}

pub enum NodeOptions {

}

pub enum EventTargetOptions {
  
}

// With a trait for running those options on a Gizmo:

//pub trait CanEditGizmo {
//  fn edit(self, &mut Gizmo);
//}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
