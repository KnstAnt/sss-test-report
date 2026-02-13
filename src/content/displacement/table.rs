use sal_core::dbg::Dbg;

pub struct Table {
    dbg: Dbg,
    header: Vec<String>,
    content: Vec<Vec<String>>,
}
//
impl Table {
    //
    pub fn new(
        parent: &Dbg,
        header: &[&str],
        content: Vec<Vec<String>>,
    ) -> Self {
        Self {
            dbg: Dbg::new(parent, "Table"),
            header: header.iter().map(|s| s.to_string()).collect(),
            content,
        }
    }
    //
    pub fn to_string(self) -> String {
        let mut string = self
            .header
            .iter()
            .map(|s| format!("|{s}"))
            .collect::<String>()
            + "|\n"
            + &(0..self.header.len()).map(|_| "|---").collect::<String>()
            + "|\n";
        self.content.iter().for_each(|v| string += &(v.iter().map(|v| format!("|{v}")).collect::<String>() + "|\n") );
        string + "  \n"
    }
}
