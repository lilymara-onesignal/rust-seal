use pair::alignment::Alignment;
use pair::alignment_matrix::AlignmentMatrix;
use pair::cursor::Cursor;
use pair::step_mask::StepMask;

pub struct Alignments<'a, T: 'a> {
    matrix: &'a T,
    stack: Vec<(StepMask, Cursor, usize)>,
    steps: Vec<StepMask>,
    score: isize,
}

impl<'a, T> Alignments<'a, T> {
    pub fn new(
        matrix: &'a T,
        stack: Vec<(StepMask, Cursor, usize)>,
        steps: Vec<StepMask>,
        score: isize,
    ) -> Self {
        Alignments {
            matrix,
            stack,
            steps,
            score,
        }
    }
}

impl<'a, T> Alignments<'a, T>
where
    T: AlignmentMatrix,
{
    fn add_branches_to_stack(&mut self, cursor: Cursor, depth: usize) {
        let steps = self.matrix.at(&cursor);

        if steps == StepMask::STOP {
            return;
        }

        for mask in [StepMask::ALIGN, StepMask::INSERT, StepMask::DELETE] {
            if steps.contains(mask) {
                let mut branch = cursor;
                branch.apply_backwards_step(mask);

                self.stack.push((mask, branch, depth));
            }
        }
    }
}

impl<'a, T> Iterator for Alignments<'a, T>
where
    T: AlignmentMatrix,
{
    type Item = Alignment;

    fn next(&mut self) -> Option<Alignment> {
        let zero = Cursor { x: 0, y: 0 };
        while let Some((step_mask, cursor, depth)) = self.stack.pop() {
            if step_mask != StepMask::STOP {
                self.steps.truncate(depth - 1);
                self.steps.push(step_mask);
            }

            self.add_branches_to_stack(cursor, depth + 1);

            if ((step_mask == StepMask::STOP) && (depth > 0)) || (cursor == zero) {
                let mut steps: Vec<StepMask> = self.steps.clone();
                steps.reverse();
                return Some(Alignment::new(cursor, steps, self.score));
            }
        }
        None
    }
}
