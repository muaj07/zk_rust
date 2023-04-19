#include "bair_witness_bridge.h"

namespace bair_witness_bridge {
  using BairWitness = ::bair_witness_bridge::BairWitness;
}

namespace bair_witness_bridge {
extern "C" {
::bair_witness_bridge::BairWitness *bair_witness_bridge$cxxbridge1$new_bair_witness() noexcept {
  ::bair_witness_bridge::BairWitness *(*new_bair_witness$)() = ::bair_witness_bridge::new_bair_witness;
  return new_bair_witness$();
}

void bair_witness_bridge$cxxbridge1$delete_bair_witness(::bair_witness_bridge::BairWitness *witness) noexcept {
  void (*delete_bair_witness$)(::bair_witness_bridge::BairWitness *) = ::bair_witness_bridge::delete_bair_witness;
  delete_bair_witness$(witness);
}
} // extern "C"
} // namespace bair_witness_bridge
