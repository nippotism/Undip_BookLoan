export const idlFactory = ({ IDL }) => {
  const LoanPayload = IDL.Record({
    'book_title' : IDL.Text,
    'borrower' : IDL.Text,
    'due_date' : IDL.Nat64,
  });
  const Loan = IDL.Record({
    'id' : IDL.Nat64,
    'borrowed_at' : IDL.Nat64,
    'book_title' : IDL.Text,
    'borrower' : IDL.Text,
    'due_date' : IDL.Nat64,
  });
  const Error = IDL.Variant({ 'NotFound' : IDL.Record({ 'msg' : IDL.Text }) });
  const Result = IDL.Variant({ 'Ok' : Loan, 'Err' : Error });
  return IDL.Service({
    'add_loan' : IDL.Func([LoanPayload], [IDL.Opt(Loan)], []),
    'delete_loan' : IDL.Func([IDL.Nat64], [Result], []),
    'get_loan' : IDL.Func([IDL.Nat64], [Result], ['query']),
    'update_loan' : IDL.Func([IDL.Nat64, LoanPayload], [Result], []),
  });
};
export const init = ({ IDL }) => { return []; };
