extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};



// #[proc_macro_derive(MockContextFunctions)]
// pub fn mock_context_functions(input: TokenStream) -> TokenStream {
//     // Parse the input tokens into a syntax tree
//     let ast = parse_macro_input!(input as DeriveInput);
//
//     // Extract the name of the struct
//     let name = &ast.ident;
//
//     // Generate the code for each function
//     let expanded = quote! {
//         impl<'a, Config: endpoint::Config> recovery::Context<Config> for #name {
//             const ENDPOINT_TYPE: endpoint::Type = Config::ENDPOINT_TYPE;
//
//             mock_context_functions!(#name, is_handshake_confirmed, active_path, active_path_mut);
//
//             // Other trait functions
//             fn path(&self) -> &super::Path<Config> {
//                 &self.path_manager[self.path_id]
//             }
//
//             fn path_mut(&mut self) -> &mut super::Path<Config> {
//                 &mut self.path_manager[self.path_id]
//             }
//
//             fn path_by_id(&self, path_id: path::Id) -> &super::Path<Config> {
//                 &self.path_manager[path_id]
//             }
//
//             fn path_mut_by_id(&mut self, path_id: path::Id) -> &mut super::Path<Config> {
//                 &mut self.path_manager[path_id]
//             }
//
//             fn path_id(&self) -> path::Id {
//                 self.path_id
//             }
//
//             fn validate_packet_ack(
//                 &mut self,
//                 _timestamp: Timestamp,
//                 _packet_number_range: &PacketNumberRange,
//                 _lowest_tracking_packet_number: PacketNumber,
//             ) -> Result<(), transport::Error> {
//                 self.validate_packet_ack_count += 1;
//
//                 if self.fail_validation {
//                     Err(transport::Error::PROTOCOL_VIOLATION)
//                 } else {
//                     Ok(())
//                 }
//             }
//
//             fn on_new_packet_ack<Pub: event::ConnectionPublisher>(
//                 &mut self,
//                 _packet_number_range: &PacketNumberRange,
//                 _publisher: &mut Pub,
//             ) {
//                 self.on_new_packet_ack_count += 1;
//             }
//
//             fn on_packet_ack(&mut self, _timestamp: Timestamp, _packet_number_range: &PacketNumberRange) {
//                 self.on_packet_ack_count += 1;
//             }
//
//             fn on_packet_loss<Pub: event::ConnectionPublisher>(
//                 &mut self,
//                 packet_number_range: &PacketNumberRange,
//                 _publisher: &mut Pub,
//             ) {
//                 self.on_packet_loss_count += 1;
//                 self.lost_packets.insert(packet_number_range.start());
//             }
//
//             fn on_rtt_update(&mut self, _now: Timestamp) {
//                 self.on_rtt_update_count += 1;
//             }
//         }
//     };
//
//     // Return the generated code as a token stream
//     TokenStream::from(expanded)
// }
