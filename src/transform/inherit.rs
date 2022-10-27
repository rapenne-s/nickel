use crate::term::RichTerm;

pub fn transform_one(rt: RichTerm) -> RichTerm {
    use crate::term::Term::RecRecord;
    if let RecRecord(fields, dyn_fields, attrs, deps, inh) = *rt.term {
    } else {
        rt
    }
}
