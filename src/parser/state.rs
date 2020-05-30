pub struct ParserState {
    input: String,
    current_slice_start: usize,
    prev_slice_start: Vec<usize>,
    current_line_start: LineStart,
    prev_line_start: Vec<LineStart>,
}

impl ParserState {
    pub fn new(input: String) -> ParserState {

        ParserState {
            input,
            current_slice_start: 0,
            prev_slice_start: vec![0],
            current_line_start: LineStart::FirstLine,
            prev_line_start: vec![],
        }
    }

    pub fn len(&self) -> usize {
        self.input.len()
    }

    pub fn current_slice(&self) -> &str {
        if self.current_slice_start >= self.len() {
            panic!("starting index of current slice exceeds length of parser input")
        }

        &self.input[self.current_slice_start..]
    }

    pub fn move_input_state_forward(&mut self, increment: usize) {
        self.move_newlines_forward(increment);
        self.move_slice_start_forward(increment);
    }

    fn move_slice_start_forward(&mut self, increment: usize) {
        self.prev_slice_start.push(self.current_slice_start);
        self.current_slice_start += increment;
    }

    fn move_newlines_forward(&mut self, increment: usize) {
        let current = self.current_slice_start;
        let new = current + increment;

        let current_slice = &self.input[current..new];
        let chars: Vec<char> = current_slice.chars().collect();
        let mut char_index = 0;

        for c in chars {
            if c == '\n' {
                self.prev_line_start.push(self.current_line_start.clone());
                
                self.current_line_start = LineStart::Index(self.current_slice_start + char_index);
            }
            
            char_index += 1;
        }
    }

    pub fn move_input_state_back(&mut self) {
        self.move_slice_start_back();
        self.move_newlines_back();
    }

    fn move_slice_start_back(&mut self) {
        self.current_slice_start = self.prev_slice_start.pop().unwrap();
    }

    fn move_newlines_back(&mut self) {
        while ParserState::line_start_is_greater_than_slice_start(self.prev_line_start.last(), self.current_slice_start) {
            self.prev_line_start.pop();
        }

        self.current_line_start = 
            self.prev_line_start.pop()
                .unwrap_or(LineStart::FirstLine);
    }

    fn line_start_is_greater_than_slice_start(line_start: Option<&LineStart>, slice_start: usize) -> bool {
        match line_start {
            Some(LineStart::Index(i)) => *i > slice_start,
            _ => false
        }
    }

    pub fn get_slice(&self, length: usize) -> Option<String> {
        let slice_end = self.current_slice_start + length;

        if slice_end > self.len() {
            None
        } else {
            let slice = &self.input[self.current_slice_start..slice_end];
            Some(String::from(slice))
        }
    }

    pub fn get_line_number(&self) -> usize {
        self.prev_line_start.len() + 1
    }

    pub fn get_column_number(&self) -> usize {
        match self.current_line_start {
            LineStart::FirstLine => self.current_slice_start + 1,
            LineStart::Index(index) => self.current_slice_start - index,
        }
    }
}

#[derive(Clone)]
pub enum LineStart {
    FirstLine,
    Index(usize),
}