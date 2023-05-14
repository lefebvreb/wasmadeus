macro_rules! all_tuples {
    ($mac: ident) => {
        $mac! { A }
        $mac! { A B }
        $mac! { A B C }
        $mac! { A B C D }
        $mac! { A B C D E }
        $mac! { A B C D E F }
        $mac! { A B C D E F G }
        $mac! { A B C D E F G H }
        $mac! { A B C D E F G H I }
        $mac! { A B C D E F G H I J }
        $mac! { A B C D E F G H I J K }
        $mac! { A B C D E F G H I J K L }
        $mac! { A B C D E F G H I J K L M }
        $mac! { A B C D E F G H I J K L M N O }
        $mac! { A B C D E F G H I J K L M N O P }
    };
}

pub(crate) use all_tuples;
