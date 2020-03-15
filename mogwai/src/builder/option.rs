//! Options for building gizmos.
//use std::collections::HashMap;
use std::sync::Arc;
use wasm_bindgen::JsValue;

use super::super::gizmo::Gizmo;
use super::super::txrx::{hand_clone, Receiver, Transmitter};


pub(crate) enum Continuous<T> {
  Rx(T, Receiver<T>),
  Static(T),
}


impl<T: Clone> Clone for Continuous<T> {
  fn clone(&self) -> Self {
    match self {
      Continuous::Rx(t, rx) => Continuous::Rx(t.clone(), hand_clone(rx)),
      Continuous::Static(t) => Continuous::Static(t.clone()),
    }
  }
}


#[derive(Clone)]
pub(crate) struct GizmoOption<T> {
  pub apply: Arc<
    Box<dyn Fn(&mut Gizmo<T>, &mut Option<Transmitter<T>>) -> Result<(), JsValue>>,
  >,
}


impl<T> GizmoOption<T> {
  pub(crate) fn new<F>(f: F) -> GizmoOption<T>
  where
    F: Fn(&mut Gizmo<T>, &mut Option<Transmitter<T>>) -> Result<(), JsValue>
      + 'static,
  {
    GizmoOption {
      apply: Arc::new(Box::new(f)),
    }
  }

  pub(crate) fn apply_option(
    self,
    gizmo: &mut Gizmo<T>,
    may_tx_post_build: &mut Option<Transmitter<T>>,
  ) -> Result<(), JsValue> {
    let option_var = self.apply;
    option_var(gizmo, may_tx_post_build)
  }
}


//#[derive(Clone)]
//pub enum GizmoBuilderOption<T> {
//  Gizmos(Continuous<Vec<GizmoBuilder<T>>>),
//  Prebuilt(T),
//  CaptureElement(Transmitter<T>),
//  WindowEvent(String, Transmitter<Event>),
//  DocumentEvent(String, Transmitter<Event>),
//
//  // Element options
//  Attribute(String, Continuous<Option<String>>),
//  Style(String, Continuous<String>),
//  Text(Continuous<String>),
//
//  // HtmlInputElement options
//  Value(Continuous<String>),
//}
//
//
//impl<T> GizmoBuilderOption<T> {
//  /// Apply the option to the built gizmo.
//  /// TODO:
//  pub(crate) fn apply(
//    self,
//    gizmo: &mut Gizmo,
//    may_tx_post_build: &mut Option<Transmitter<T>>,
//  ) -> Result<(), JsValue> {
//    use Continuous::*;
//    use GizmoBuilderOption::*;
//    match self {
//      Attribute(name, Rx(init, dynamic)) => {
//        gizmo.attribute(&name, init.clone(), dynamic.branch());
//        Ok(())
//      }
//
//      Style(name, Static(value)) => gizmo
//        .element
//        .dyn_ref::<T>()
//        .ok_or(JsValue::NULL)?
//        .style()
//        .set_property(&name, &value),
//
//      Style(name, Rx(init, dynamic)) => {
//        gizmo.style(&name, &init, dynamic.branch());
//        Ok(())
//      }
//
//      Text(Static(value)) => {
//        let text: web_sys::Text =
//          web_sys::Text::new_with_data(&value).unwrap_throw();
//        (gizmo.as_ref() as &Node).append_child(text.as_ref())?;
//        Ok(())
//      }
//
//      Text(Rx(init, dynamic)) => {
//        gizmo.text(&init, dynamic.branch());
//        Ok(())
//      }
//
//      Value(Static(value)) => {
//        gizmo
//          .element
//          .dyn_ref::<T>()
//          .expect("Attempted to set the value of non-input gizmo element.")
//          .set_value(&value);
//        Ok(())
//      }
//
//      Value(Rx(init, rx)) => {
//        gizmo
//          .element
//          .dyn_ref::<HtmlInputElement>()
//          .expect("Attempted to set the value of non-input gizmo element.")
//          .set_value(&init);
//        gizmo.value(&init, rx.branch());
//        Ok(())
//      }
//
//      Gizmos(Static(static_gizmo_builders)) => {
//        let static_gizmos: Vec<_> =
//          static_gizmo_builders.into_iter().fold(
//          Ok(vec![]),
//          |res: Result<_, JsValue>, builder| {
//            let mut gizmos = res?;
//            let gizmo = builder.build()?;
//            gizmos.push(gizmo);
//            Ok(gizmos)
//          },
//        )?;
//
//        let node = (gizmo.as_ref() as &Node).clone();
//        static_gizmos.into_iter().fold(Ok(()), |res, static_gizmo| {
//          res?;
//          node.append_child(&static_gizmo.element)?;
//          gizmo.static_gizmos.push(static_gizmo);
//          Ok(())
//        })
//      }
//
//      Gizmos(Rx(init_builders, rx)) => {
//        let init_gizmos =
//          init_builders.into_iter().fold(
//          Ok(vec![]),
//          |res: Result<_, JsValue>, builder| {
//            let mut gizmos = res?;
//            let gizmo = builder.build()?;
//            gizmos.push(gizmo);
//            Ok(gizmos)
//          },
//        )?;
//
//        gizmo.gizmos(init_gizmos, rx.branch());
//        Ok(())
//      }
//
//      Prebuilt(el) => {
//        (gizmo.as_ref() as &Node)
//          .dyn_ref::<Node>()
//          .ok_or(JsValue::NULL)?
//          .append_child(el.dyn_ref().ok_or(JsValue::NULL)?)?;
//        Ok(())
//      }
//
//      CaptureElement(tx_pb) => {
//        may_tx_post_build = Some(tx_pb);
//        Ok(())
//      }
//
//      WindowEvent(ev, tx) => {
//        gizmo.window_tx_on(&ev, tx);
//        Ok(())
//      }
//
//      DocumentEvent(ev, tx) => {
//        gizmo.document_tx_on(&ev, tx);
//        Ok(())
//      }
//    }
//  }
//}
