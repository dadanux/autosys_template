
use regex::Regex;
use std::collections::HashMap;


pub fn process_text(source: &str, replacements: &HashMap<String, String>) -> String {
    let mut output_lines = Vec::new();

    let machine_re = Regex::new(r#"machine\s*:\s*"?([^"\s]+)"?"#).unwrap();
    let insert_job_re = Regex::new(r#"insert_job: [^-]+-(.+)"#).unwrap();
    let box_name_re = Regex::new(r"-([^-\n]+.*)").unwrap();
    let owner_re = Regex::new(r#"owner\s*:\s*"?([^"\s]+)"?"#).unwrap();
    for line in source.lines() {
        let mut new_line = line.to_string();

        // if line.starts_with("owner:") {
        //          if let Some(caps) = owner_re.captures(line) {
        //         if let Some(owner) = caps.get(1) {
        //             new_line = format!("owner: {{{{ {} }}}}", owner.as_str());
        //         }
        //     }
        // } else 
        if line.starts_with("insert_job:") {
            if let Some(caps) = insert_job_re.captures(line) {
                if let Some(job_name) = caps.get(1) {
                    new_line = format!("insert_job: {{{{ prefix }}}}-{}", job_name.as_str());
                }
            }
        } else if line.starts_with("box_name:") {
            if let Some(caps) = box_name_re.captures(line) {
                if let Some(box_name) = caps.get(1) {
                    new_line = format!("box_name: {{{{ prefix }}}}-{}", box_name.as_str());
                }
            }
        }  

        new_line = machine_re.replace_all(&new_line, |caps: &regex::Captures| {
            let name = caps.get(1).map(|m| m.as_str().trim()).unwrap_or("");
            if let Some(rep) = replacements.get(name) {
                if !rep.is_empty() {
                    return format!("machine: {{{{ prefix | lower }}}}_{{{{ {} }}}}_a", rep);
                }
            }
            caps.get(0).unwrap().as_str().to_string()
        }).to_string();

        new_line = owner_re.replace_all(&new_line, |caps: &regex::Captures| {
            let name = caps.get(1).map(|m| m.as_str().trim()).unwrap_or("");
            if let Some(rep) = replacements.get(name) {
                if !rep.is_empty() {
                    return format!("owner: {{{{ {} }}}}", rep);
                }
            }
            caps.get(0).unwrap().as_str().to_string()
        }).to_string();

        output_lines.push(new_line);
    }

    output_lines.join("\n")
}


// pub fn process_text(source: &str, replacements: &HashMap<String, String>) -> String {
//     let mut output_lines = Vec::new();

//     let machine_re = Regex::new(r#"(?i)machine\s*[:=]\s*"?([^"]+?)"?"#).unwrap();
//     let insert_job_re = Regex::new(r#"insert_job:\s*([^\s]+)"#).unwrap();
//     let box_name_re = Regex::new(r#"box_name\s*:\s*"?([^"\s]+)"?"#).unwrap();

//     for line in source.lines() {
//         let mut new_line = line.to_string();

//         if line.starts_with("owner:") {
//             new_line = "owner: {{ owner }}".to_string();
//         } else if line.starts_with("insert_job:") {
//             if let Some(caps) = insert_job_re.captures(line) {
//                 if let Some(job_name) = caps.get(1) {
//                     new_line = format!("insert_job: {{ prefix }}_{}", job_name.as_str());
//                 }
//             }
//         } else if line.starts_with("box_name:") {
//             if let Some(caps) = box_name_re.captures(line) {
//                 if let Some(box_name) = caps.get(1) {
//                     new_line = format!("box_name: {{ prefix }}_{}", box_name.as_str());
//                 }
//             }
//         }

//         new_line = machine_re.replace_all(&new_line, |caps: &regex::Captures| {
//             let name = caps.get(1).map(|m| m.as_str().trim()).unwrap_or("");
//             if let Some(rep) = replacements.get(name) {
//                 if !rep.is_empty() {
//                     return format!("machine: {{ prefix | lower }}_{{{}_a}}", rep);
//                 }
//             }
//             caps.get(0).unwrap().as_str().to_string()
//         }).to_string();

//         output_lines.push(new_line);
//     }

//     output_lines.join("\n")
// }
