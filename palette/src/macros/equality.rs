macro_rules! impl_eq {
    (  $self_ty: ident , [$($element: ident),+]) => {
        impl_eq!($self_ty<>, [$($element),+]);
    };
    (  $self_ty: ident < $($ty_param: ident),* > , [$($element: ident),+]) => {

        impl<$($ty_param,)* T> PartialEq for $self_ty<$($ty_param,)* T>
        where
            T: PartialEq,
        {
            fn eq(&self, other: &Self) -> bool {
                $( self.$element == other.$element )&&+
            }
        }

        impl<$($ty_param,)* T> Eq for $self_ty<$($ty_param,)* T> where T: Eq {}

        impl<$($ty_param,)* T> AbsDiffEq for $self_ty<$($ty_param,)* T>
        where T: AbsDiffEq,
            T::Epsilon: Clone,
        {
            type Epsilon = T::Epsilon;

            fn default_epsilon() -> Self::Epsilon {
                T::default_epsilon()
            }

            fn abs_diff_eq(&self, other: &Self, epsilon: T::Epsilon) -> bool {
                $( self.$element.abs_diff_eq(&other.$element, epsilon.clone()) )&&+
            }
            fn abs_diff_ne(&self, other: &Self, epsilon: T::Epsilon) -> bool {
                $( self.$element.abs_diff_ne(&other.$element, epsilon.clone()) )||+
            }
        }

        impl<$($ty_param,)* T> RelativeEq for $self_ty<$($ty_param,)* T>
        where T: RelativeEq,
            T::Epsilon: Clone,
        {
            fn default_max_relative() -> T::Epsilon {
                T::default_max_relative()
            }

            fn relative_eq(&self, other: &Self, epsilon: T::Epsilon, max_relative: T::Epsilon) -> bool {
                $( self.$element.relative_eq(&other.$element, epsilon.clone(), max_relative.clone()) )&&+
            }
            fn relative_ne(&self, other: &Self, epsilon: T::Epsilon, max_relative: T::Epsilon) -> bool {
                $( self.$element.relative_ne(&other.$element, epsilon.clone(), max_relative.clone()) )||+
            }
        }

        impl<$($ty_param,)* T> UlpsEq for $self_ty<$($ty_param,)* T>
        where T: UlpsEq,
            T::Epsilon: Clone,
        {
            fn default_max_ulps() -> u32 {
                T::default_max_ulps()
            }

            fn ulps_eq(&self, other: &Self, epsilon: T::Epsilon, max_ulps: u32) -> bool {
                $( self.$element.ulps_eq(&other.$element, epsilon.clone(), max_ulps) )&&+
            }
            fn ulps_ne(&self, other: &Self, epsilon: T::Epsilon, max_ulps: u32) -> bool {
                $( self.$element.ulps_ne(&other.$element, epsilon.clone(), max_ulps) )||+
            }
        }
    }
}

macro_rules! impl_eq_hue {
    (  $self_ty: ident, $hue_ty: ident, [$($element: ident),+]) => {
        impl_eq_hue!($self_ty<>, $hue_ty, [$($element),+]);
    };
    (  $self_ty: ident < $($ty_param: ident),* >, $hue_ty: ident, [$($element: ident),+]) => {

        impl<$($ty_param,)* T> PartialEq for $self_ty<$($ty_param,)* T>
        where
            T: PartialEq,
            $hue_ty<T>: PartialEq,
        {
            fn eq(&self, other: &Self) -> bool {
                $( self.$element == other.$element )&&+
            }
        }

        impl<$($ty_param,)* T> Eq for $self_ty<$($ty_param,)* T>
        where
            T: Eq,
            $hue_ty<T>: Eq,
        {}

        impl<$($ty_param,)* T> AbsDiffEq for $self_ty<$($ty_param,)* T>
        where
            T: AbsDiffEq,
            T::Epsilon: Clone,
            $hue_ty<T>: AbsDiffEq<Epsilon = T::Epsilon>,
        {
            type Epsilon = T::Epsilon;

            fn default_epsilon() -> Self::Epsilon {
                T::default_epsilon()
            }

            fn abs_diff_eq(&self, other: &Self, epsilon: T::Epsilon) -> bool {
                $( self.$element.abs_diff_eq(&other.$element, epsilon.clone()) )&&+
            }
            fn abs_diff_ne(&self, other: &Self, epsilon: T::Epsilon) -> bool {
                $( self.$element.abs_diff_ne(&other.$element, epsilon.clone()) )||+
            }
        }

        impl<$($ty_param,)* T> RelativeEq for $self_ty<$($ty_param,)* T>
        where
            T: RelativeEq,
            T::Epsilon: Clone,
            $hue_ty<T>: RelativeEq + AbsDiffEq<Epsilon = T::Epsilon>,
        {
            fn default_max_relative() -> T::Epsilon {
                T::default_max_relative()
            }

            fn relative_eq(&self, other: &Self, epsilon: T::Epsilon, max_relative: T::Epsilon) -> bool {
                $( self.$element.relative_eq(&other.$element, epsilon.clone(), max_relative.clone()) )&&+
            }
            fn relative_ne(&self, other: &Self, epsilon: T::Epsilon, max_relative: T::Epsilon) -> bool {
                $( self.$element.relative_ne(&other.$element, epsilon.clone(), max_relative.clone()) )||+
            }
        }

        impl<$($ty_param,)* T> UlpsEq for $self_ty<$($ty_param,)* T>
        where
            T: UlpsEq,
            T::Epsilon: Clone,
            $hue_ty<T>: UlpsEq + AbsDiffEq<Epsilon = T::Epsilon>,
        {
            fn default_max_ulps() -> u32 {
                T::default_max_ulps()
            }

            fn ulps_eq(&self, other: &Self, epsilon: T::Epsilon, max_ulps: u32) -> bool {
                $( self.$element.ulps_eq(&other.$element, epsilon.clone(), max_ulps) )&&+
            }
            fn ulps_ne(&self, other: &Self, epsilon: T::Epsilon, max_ulps: u32) -> bool {
                $( self.$element.ulps_ne(&other.$element, epsilon.clone(), max_ulps) )||+
            }
        }
    }
}
