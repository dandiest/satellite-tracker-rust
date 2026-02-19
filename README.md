<p align="center">
  <img src="https://img.shields.io/badge/License-MIT-yellow.svg" />
  <img src="https://img.shields.io/badge/Language-Rust-orange.svg" />
  <img src="https://img.shields.io/badge/Status-Stable-brightgreen.svg" /> 
  <img src="https://img.shields.io/badge/Library-Serde-blue.svg" />
  <img src="https://img.shields.io/badge/Library-Chrono-red.svg" />
</p>

<h1 align="center">🛰️ Satellite Tracker CLI - Conditional State Manager</h1>

<p align="center">
  A robust Rust engine designed for monitoring satellite transmission codes, featuring state awareness and intelligent data synchronization logic.
</p>

---

## 🎓 Educational Disclaimer
This repository is a key milestone in my **Personal Apprenticeship** with Rust.
* **Purpose**: To master "State-Aware" programming, where application behavior is dictated by previously persisted data.
* **Focus**: Deep dive into **Mutable References**, **Data Sanitization**, and **Conditional Persistence**.

## 🌟 Features
* **Intelligent Update Logic**: The system compares incoming signals with stored data to prevent redundant disk I/O operations.
* **State Persistence**: Full lifecycle management of a JSON-based database using `Serde`.
* **Abstraction via Functions**: Implements custom input handling to ensure DRY (Don't Repeat Yourself) code principles.
* **Audit-Ready Logs**: Automatic UTC timestamping for every successful state transition.

## 🛠️ Technical Deep Dive
* **Mutable Struct Mutation**: Leverages Rust's ownership model to update existing structures in-place, optimizing memory usage.
* **Input Sanitization**: Advanced use of `.trim()` to bridge the gap between raw CLI buffers and clean, comparable strings.
* **Error Resilience**: Uses exhaustive `match` patterns to handle "First Run" scenarios and file system interruptions gracefully.

---

## 🚀 How to Run
1. Clone the repository.
2. Build and run via Cargo:
   ```bash
   cargo run

## ⚖️ License & Copyright

Copyright © 2026 **[dandiest]**

*Licensed under the MIT License.*
