use super::Content;

pub struct General {
}
//
impl General {
    pub fn new() -> Self {
        Self {}
    }
}
//
impl Content for General {
    //
    fn to_string(self) -> String {
"# Общие данные
Тестовые расчеты предназначены для проверки ПО при ежегодном, промежуточном и возобновляющем освидетельствовании на борту судна в присутствии инспектора РС или в другом необходимом случае.
Для каждого тестового расчета приведены результаты расчета с использованием ПО и сравнение значений с приведенными в Документации по остойчивости/прочности на судно.  

В таблицах с тестовыми расчетами приведено:  
- в столбце \"Документация\" приведены значения в соответствии с одобренной Документацией по остойчивости/прочности;  
- в столбце \"Расчет\" приведено значение, полученное в результате расчета в ПО;  
- в столбце \"%\" приведена величина погрешности, которая определяется по формуле:  
1.  для расчетов перерезывающих сил и изгибающих моментов:  
> % = (базовое значение – рассчитанное значение) / допустимое значение х 100
2. для расчетов остойчивости:  
> % = (базовое значение – рассчитанное значение) / базовое значение х 100

В качестве базового значения принимается значение из одобренной Документации по остойчивости/прочности. В случае если указано два значения допустимой погрешности, в качестве допустимой величины принимается большее из них.  
- в столбце \"Допуск\" приведена допускаемая величина погрешности в соответствии с Правилами Регистра.  
- в столбце \"Статус\" указан признак превышения полученной погрешности над допустимым значением.  \n".to_string()
    }
}
