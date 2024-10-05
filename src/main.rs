use knitting_interpret::graph::DirectedKnittingGraph;

enum Stitch {
    Knit,
    Purl,
    CastOn,
}

fn main() {
    let mut fabric = DirectedKnittingGraph::new();

    fabric.cast_on(4);
    fabric.cast_on(1);
    fabric.turn();
    fabric.knit();
    fabric.knit();
    fabric.knit();
    fabric.knit();
    fabric.knit();
    fabric.turn();
    fabric.knit();
    fabric.knit();
    fabric.knit();

    println!("{}", fabric.dot());

}
