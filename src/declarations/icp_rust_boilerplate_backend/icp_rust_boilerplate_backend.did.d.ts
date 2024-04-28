import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export type Error = { 'NotFound' : { 'msg' : string } };
export interface Loan {
  'id' : bigint,
  'borrowed_at' : bigint,
  'book_title' : string,
  'borrower' : string,
  'due_date' : bigint,
}
export interface LoanPayload {
  'book_title' : string,
  'borrower' : string,
  'due_date' : bigint,
}
export type Result = { 'Ok' : Loan } |
  { 'Err' : Error };
export interface _SERVICE {
  'add_loan' : ActorMethod<[LoanPayload], [] | [Loan]>,
  'delete_loan' : ActorMethod<[bigint], Result>,
  'get_loan' : ActorMethod<[bigint], Result>,
  'update_loan' : ActorMethod<[bigint, LoanPayload], Result>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: ({ IDL }: { IDL: IDL }) => IDL.Type[];
