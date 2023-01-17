#[cfg(test)]
mod tests{
    #[test]
    fn case_sensitive(){
        let query = "hola";
        let contents = "\
        hola, chao, wena
        sisisisiisisi";

        assert_eq!(vec!["hola, chao, wena"], crate::search(query,contents))
    }

    #[test]
    fn case_insensitive(){
        let query = "hOLa";
        let contents = "\
        hola, chao, wena
        sisisisiisisi";

        assert_eq!(vec!["hola, chao, wena"], crate::search_case_insensitive(query,contents))
    }
}