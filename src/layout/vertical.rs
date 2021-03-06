//! A wrapper for `NSLayoutAnchorX`, which is typically used to handle values for how a 
//! given view should layout along the x-axis. Of note: the only thing that can't be protected
//! against is mixing/matching incorrect left/leading and right/trailing anchors. Be careful!

use objc::{msg_send, sel, sel_impl};
use objc::runtime::Object;
use objc_id::ShareId;

use crate::foundation::id;
use crate::layout::constraint::LayoutConstraint;

/// A wrapper for `NSLayoutAnchor`. You should never be creating this yourself - it's more of a
/// factory/helper for creating `LayoutConstraint` objects based on your views.
#[derive(Clone, Debug, Default)]
pub struct LayoutAnchorY(pub Option<ShareId<Object>>);

impl LayoutAnchorY {
    /// An internal method for wrapping existing constraints.
    pub(crate) fn new(object: id) -> Self {
        LayoutAnchorY(Some(unsafe {
            ShareId::from_ptr(object)
        }))
    }

    /// Return a constraint equal to another vertical anchor.
    pub fn constraint_equal_to(&self, anchor_to: &LayoutAnchorY) -> LayoutConstraint {
        match (&self.0, &anchor_to.0) {
            (Some(from), Some(to)) => LayoutConstraint::new(unsafe {
                let b: id = msg_send![*from, constraintEqualToAnchor:&**to];
                b
            }),

            _ => { panic!("Attempted to create vertical constraints with an uninitialized anchor!"); }
        }
    }

    /// Return a constraint greater than or equal to another vertical anchor.
    pub fn constraint_greater_than_or_equal_to(&self, anchor_to: &LayoutAnchorY) -> LayoutConstraint {
        match (&self.0, &anchor_to.0) {
            (Some(from), Some(to)) => LayoutConstraint::new(unsafe {
                msg_send![*from, constraintGreaterThanOrEqualToAnchor:&**to]
            }),

            _ => { panic!("Attempted to create vertical constraints with an uninitialized anchor!"); }
        }
    }

    /// Return a constraint less than or equal to another vertical anchor.
    pub fn constraint_less_than_or_equal_to(&self, anchor_to: &LayoutAnchorY) -> LayoutConstraint {
        match (&self.0, &anchor_to.0) {
            (Some(from), Some(to)) => LayoutConstraint::new(unsafe {
                msg_send![*from, constraintLessThanOrEqualToAnchor:&**to]
            }),

            _ => { panic!("Attempted to create vertical constraints with an uninitialized anchor!"); }
        }
    }
}
