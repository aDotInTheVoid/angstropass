use angstropass::langs;

#[langs]
mod langs {
    mod src {
        // Terminals
        type Pr = crate::Primitive;
        type X = crate::Symbol;
        type C = crate::Constant;
        type D = crate::Datum;

        enum Expr {
            Primitive(Pr),
            Symbol(X),
            Constant(C),
            Quote(D),
            If(Box<Expr>, Box<Expr>, Box<Expr>),
            If2(Box<Expr>, Box<Expr>),
            Or(Vec<Expr>),
            And(Vec<Expr>),
        }
    }

    #[extends(src)]
    mod l1 {
        #[replace]
        type Pr = crate::PrimitiveOrVoid;

        enum Expr {
            #[delete]
            If2,
        }
    }
}

fn main() {}
