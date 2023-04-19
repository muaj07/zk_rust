#pragma once
#include "../../libstark/src/languages/Bair/BairWitness.hpp"

namespace bair_witness_bridge {
  using BairWitness = libstark::BairWitness;

  BairWitness *new_bair_witness();
  void delete_bair_witness(BairWitness *witness);
}
