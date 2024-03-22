use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct JSONLD<'a, S: Serialize, T: Serialize> {
    #[serde(rename(serialize = "@context"))]
    context: &'a S,
    #[serde(rename(serialize = "@graph"))]
    graph: &'a T,
}

impl<'a, S: Serialize, T: Serialize> JSONLD<'a, S, T> {
    pub fn new(context: &'a S, graph: &'a T) -> Self {
        JSONLD { context, graph }
    }
}
