use knitting_interpret::graph::{
    KnitError,
    DirectedKnittingGraph
};

enum Stitch {
    Knit,
    Purl,
    CastOn,
    K2Tog,
}

fn main() {

    let fabric = knit_test().expect("Knitting instructions invalid");

    println!("{}", fabric.dot());

}

fn knit_test() -> Result<DirectedKnittingGraph, KnitError> {
    let mut fabric = DirectedKnittingGraph::new();

    fabric.cast_on(4);
    fabric.cast_on(1);
    fabric.turn();
    fabric.knit()?;
    fabric.knit()?;
    fabric.knit()?;
    fabric.knit()?;
    fabric.knit()?;
    fabric.turn();
    fabric.knit()?;
    fabric.k2tog()?;
    fabric.knit()?;
    fabric.knit()?;

    Ok(fabric)
}
