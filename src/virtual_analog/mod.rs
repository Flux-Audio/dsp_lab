//! This module contains emulations of various physical analog electronic components
//! for use in virtual analog synthesis. Note that these components are not
//! accurate to real analog circuits, and in fact it's not possible to build a
//! correct circuit topology, it's merely a collection of atomic effects meant
//! to simulate the artifacts of real components in signal processing.

pub mod physics;        // defines basic physical laws used across all components

// TODO: place into a "components" sub-module
// pub mod capacitors   // TODO: real capacitor modelling, various levels of detail, with thermal noise
// pub mod resistors    // TODO: real resistor modelling, various levels of detail, with thermal noise
// pub mod inductors    // TODO: real inductor modelling, various levels of detial, with thermal noise
// pub mod diodes       // TODO: real diode modelling
// pub mod vactrols     // TODO: real vactrol modelling
// pub mod transformers // TODO: real voltage transformer modelling
// pub mod amps         // TODO: real differential amplifier modelling, op-amps, comparators, schmitt triggers.
// pub mod transistor   // TODO: real transistor modelling
// pub mod tubes        // TODO: real vacuum tube modelling
// pub mod dischargers  // TODO: real discharge tube modelling, like stroboscopic tubes, fluorescent tubes

// pub mod circuits     // TODO: circuits that use the other components