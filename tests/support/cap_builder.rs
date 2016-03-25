use term::terminfo::{parm, TermInfo};

// Use like:
//
// let bytes: Vec<u8> = CapBuilder::new(&terminfo)
//     .cap("cup")
//     .number_param(13)
//     .number_param(12)
//     .build()
//
pub struct CapBuilder<'a, 'b> {
    terminfo: &'a TermInfo,
    cap: Option<String>,
    variables: Option<&'b mut parm::Variables>,
    params: Vec<parm::Param>,
}

impl<'a, 'b> CapBuilder<'a, 'b> {
    pub fn new(terminfo: &'a TermInfo) -> CapBuilder<'a, 'b> {
        CapBuilder {
            terminfo: terminfo,
            cap: None,
            variables: None,
            params: vec![],
        }
    }

    pub fn cap<S>(mut self, cap: S) -> CapBuilder<'a, 'b>
        where S: Into<String>
    {
        self.cap = Some(cap.into());
        self
    }

    #[allow(dead_code)]
    pub fn variables(mut self,
                     variables: &'b mut parm::Variables)
                     -> CapBuilder<'a, 'b> {
        self.variables = Some(variables);
        self
    }

    pub fn number_param(mut self, val: i32) -> CapBuilder<'a, 'b> {
        self.params.push(parm::Param::Number(val));
        self
    }

    #[allow(dead_code)]
    pub fn word_param<S>(mut self, val: S) -> CapBuilder<'a, 'b>
        where S: Into<String>
    {
        self.params.push(parm::Param::Words(val.into()));
        self
    }

    pub fn build(self) -> Result<Vec<u8>, String> {
        if self.variables.is_some() {
            self.build_with_variables()
        } else {
            self.build_without_variables()
        }
    }

    fn build_with_variables(self) -> Result<Vec<u8>, String> {
        let variables = try! { self.variables.ok_or("oops, expected variables to be defined") };
        let cap = try! { self.cap.ok_or("invalid configuration: cap not provided") };
        let cmd = try! { self.terminfo.strings.get(&cap).ok_or("cap doesn't exist") };

        parm::expand(&cmd, self.params.as_slice(), variables)
            .or_else(|e| Err(format!("error expanding: {}", e)))
    }

    fn build_without_variables(self) -> Result<Vec<u8>, String> {
        let mut variables = parm::Variables::new();
        let cap = try! { self.cap.ok_or("invalid configuration: cap not provided") };
        let cmd = try! { self.terminfo.strings.get(&cap).ok_or("cap doesn't exist") };
        parm::expand(&cmd, self.params.as_slice(), &mut variables)
            .or_else(|e| Err(format!("error expanding: {}", e)))
    }
}
