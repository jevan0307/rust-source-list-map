extern crate source_list_map;

mod utils;

#[cfg(test)]
mod map_generated_code {
    use source_list_map::*;
    use utils::*;

    #[test]
    fn should_map_generated_code_correctly() {
        let mut map = SourceListMap::new(None, None, None);

		let source: String = vec![
			"Normal Line 1",
			"Normal Line 2",
			"$",
			"Normal Line 3",
			"Line A;Line B;Line C",
			"Line A;Line B;Line C",
			"No\\",
			"New\\",
			"Line 1",
			"No\\",
			"$",
			"New\\",
			"$",
			"$",
			"Line 2",
			"End Line"
		].join("\n");

		map.add(Node::NString(source.clone() + "\n"),
                Some(g_Str("file.txt")),
                Some(source.clone() + "\n"));
        map.add(Node::NString(source.clone() + "\n"),
                Some(g_Str("file.txt")),
                Some(source.clone() + "\n"));
        map.add(Node::NString(source.clone() + "\n"),
                None,
                None);
        map.add(Node::NString(source.clone()),
                Some(g_Str("file.txt")),
                Some(source.clone() + "\n"));

		let mut new_map = map.map_generated_code(&|line| {
            line.replace(";", "\n").replace("\\\n", " ").replace("$\n", "")
        });
		let result = new_map.to_string_with_source_map(Some(g_Str("test.txt")));
		let expected_part = vec![
			"AACA",
			"AAEA",
			"AACA",
			"AAAA",
			"AAAA",
			"AACA",
			"AAAA",
			"AAAA",
			"AACA,GACA,IACA",
			"AACA,GAEA,IAGA",
			"AACA"
		].join(";");

        assert_eq!(result.map.mappings,
                    vec![
                		"AAAA",
                		&expected_part,
                		"AAfA",
                		&expected_part,
                		";;;;;;;;;;;",
                		"AAfA",
                		&expected_part
                	].join(";"));

        assert_eq!(result.source,
                    vec![
                        source.as_str(),
                        source.as_str(),
                        source.as_str(),
                        source.as_str()]
                    .join("\n")
                    .replace(";", "\n")
                    .replace("\\\n", " ")
                    .replace("$\n", ""));
    }

    #[test]
    fn should_map_code_with_many_lines_in_time() {
        // TODO: Enhance performance and increase repeat to 200000
        let source = "MyLine\n".repeat(10000);

        let mut map = SourceListMap::new(None, None, None);
        map.add(Node::NString(source.clone()), Some(g_Str("file.txt")), Some(source.clone()));
        let mut new_map = map.map_generated_code(&|line| line);
        let result = new_map.to_string_with_source_map(Some(g_Str("test.txt")));

        assert_eq!(result.source, source);
        assert_eq!(result.map.sources_content[0], source);
    }
}
