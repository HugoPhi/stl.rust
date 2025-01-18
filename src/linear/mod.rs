pub mod rc_linked_list;
pub mod box_linked_list;


// provide to up crates, if you want to use box impled, use: 
//
// ```rust
// pub use box_linked_list::*;
// ```
//
// here
// pub use rc_linked_list::*;
pub use box_linked_list::*;
