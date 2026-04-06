/*
 * VRYPTIC Hardware Abstraction Layer (HAL)
 * -----------------------------------------
 * This interface defines the secure handshake between the CMOS sensor 
 * and the Solana Seed Vault / AOSP StrongBox.
 */

#ifndef VRYPTIC_HAL_H
#define VRYPTIC_HAL_H

#include <stdint.h>

typedef struct {
    uint8_t fingerprint[64];    // Unique silicon noise ID (PRNU-derived)
    uint64_t capture_epoch;     // Verified hardware timestamp
    uint32_t confidence_score;  // Probabilistic match rating
} NpccSeal;

// Initiates Asynchronous Hardware Attestation (AHA)
NpccSeal perform_hardware_handshake(void* raw_frame_buffer);

// Bridges the NpccSeal to the Solana Seed Vault for signing
int anchor_to_seed_vault(NpccSeal* seal);

#endif // VRYPTIC_HAL_H
