#[macro_export]
macro_rules! GENERAL_CONDUCTOR_DATA { () => { 
r#"persistence_dir = "{}/persistence"
#signing_service_uri = "http://localhost:8888"

[network]
type="n3h"
n3h_persistence_path = "{}/n3h"
n3h_log_level = 't'
#bootstrap_nodes = []
#n3h_mode = "REAL"
#Agent for hosting applications"#
}; }

#[macro_export]
macro_rules! DNA_STRING { () => { 
r#"[[dnas]]
file = "{}"
id = "{}"
hash = "{}""#
}; }

#[macro_export]
macro_rules! AGENT_STRING { () => { 
r#"[[agents]]
id = "{}"
name = "{}"
keystore_file = "{}"
public_address = "{}""# 
}; }

#[macro_export]
macro_rules! INSTANCE_STRING { () => {  
r#"[[instances]]
agent = "{}"
dna = "{}"
id = "{}-{}"
[instances.storage]
path = "/holochain/{}/storage/{}"
type = "file""# 
}; }

#[macro_export]
macro_rules! INTERFACE_STRING { () => { 
"\t[[interfaces.instances]]
\tid = \"{}-{}\""
}; }

pub static INTERFACE_GENERAL: &str = "
[[interfaces]]
id = \"http interface\"
admin = true";

pub static INTERFACE_FINAL: &str = "
\t[interfaces.driver]
\ttype = \"http\"
\tport = 4000";
