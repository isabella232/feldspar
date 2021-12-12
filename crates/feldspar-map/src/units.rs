macro_rules! def_units {
    ($t: ident) => {
        /// Denotes that the inner `T` is given in units of voxels.
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub struct $t<T>(pub T);

        impl<T> $t<T> {
            pub fn into_inner(self) -> T {
                self.0
            }

            pub fn map<S>(self, mut f: impl FnMut(T) -> S) -> $t<S> {
                $t(f(self.0))
            }

            pub fn combine<S, R>(self, other: $t<S>, mut f: impl FnMut(T, S) -> R) -> $t<R> {
                $t(f(self.into_inner(), other.into_inner()))
            }
        }
    };
}

def_units!(VoxelUnits);
def_units!(ChunkUnits);
