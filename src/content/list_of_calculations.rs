use super::Content;

pub struct ListOfCalculations {
    content: Vec<String>,
}
//
impl ListOfCalculations {
    pub fn new(content: Vec<&str>) -> Self {
        Self {
            content: content.iter().map(|s| s.to_string()).collect(),
        }
    }
}
//
impl Content for ListOfCalculations {
    //
    fn to_string(self) -> String {
        let header = "|№|Наименование случая|Исходный документ|\n|---|---|---|\n".to_owned();
        let enumerated = self.content.into_iter().enumerate();
        let col1: Vec<String> = enumerated
            .clone()
            .filter(|(i, _)| i % 2 == 0)
            .map(|(_, v)| v)
            .collect();
        let col2: Vec<String> = enumerated
            .filter(|(i, _)| i % 2 != 0)
            .map(|(_, v)| v)
            .collect();
        let body = col1
            .iter()
            .zip(col2.iter())
            .enumerate()
            .map(|(i, (s1, s2))| format!("|{}|{}|{}|\n", (i + 1), s1, s2))
            .collect::<String>();
        "# Перечень тестовых расчетов  \n".to_string() + &header + &body + "\n"
    }
}
