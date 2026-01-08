/// Trait for integer types that can be used in QueueWithIntervals.
///
/// This trait is designed exclusively for integer types (i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, isize, usize).
/// Float types (f32, f64) are NOT supported and cannot implement this trait.
///
/// The trait provides basic arithmetic operations and checked operations needed for queue interval management.
pub trait QueueValue:
    Copy
    + Clone
    + Ord
    + PartialOrd
    + PartialEq
    + std::fmt::Debug
    + std::fmt::Display
    + std::ops::Add<Output = Self>
    + std::ops::Sub<Output = Self>
    + std::ops::AddAssign
    + std::ops::SubAssign
{
    /// Returns the value representing zero
    fn zero() -> Self;

    /// Returns the value representing one
    fn one() -> Self;

    /// Safely subtracts one, handling underflow for unsigned types.
    /// Returns None if the operation would underflow.
    fn checked_sub_one(self) -> Option<Self>;

    /// Safely adds one, handling overflow.
    /// Returns None if the operation would overflow.
    fn checked_add_one(self) -> Option<Self>;
}

macro_rules! impl_queue_value {
    ($($t:ty),*) => {
        $(
            impl QueueValue for $t {
                fn zero() -> Self {
                    0
                }

                fn one() -> Self {
                    1
                }

                fn checked_sub_one(self) -> Option<Self> {
                    self.checked_sub(1)
                }

                fn checked_add_one(self) -> Option<Self> {
                    self.checked_add(1)
                }
            }
        )*
    };
}

// Implement QueueValue for all standard Rust integer types
// Signed integers: i8, i16, i32, i64, i128, isize
// Unsigned integers: u8, u16, u32, u64, u128, usize
impl_queue_value!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);
