# Stellar Attendance Smart Contract
contract id = CDH75IYDEYHLGJEBAVYIZO5TJ5WWLF2XPW525OYAIWJWFW3O6VANMSQN

**Stellar Attendance System** - Blockchain-Based Decentralized Event Presences & POAP System

## Project Description

Stellar Attendance System is a decentralized smart contract solution built on the Stellar blockchain using the Soroban SDK. It provides a secure and immutable platform for recording event, class, or bootcamp attendance directly on the blockchain. 

By replacing fragile paper-based systems or centralized databases (like Google Forms), this contract ensures that attendance data (recording NIM/Student ID and Name) is transparent, tamper-proof, and reliably stored. This project focuses purely on the backend/smart contract logic, allowing interactions and testing directly via Soroban Studio or the Soroban CLI.

## Project Vision

Our vision is to revolutionize event management and academic credentialing by:

- **Decentralizing Data**: Moving attendance records from centralized servers to a global, distributed ledger.
- **Guaranteeing Immutability**: Providing a permanent, tamper-proof record of attendance that cannot be altered, forged, or manipulated by third parties.
- **Ensuring Ownership**: Paving the way for students to own their attendance records as verifiable digital portfolios or POAPs (Proof of Attendance Protocols).
- **Building Trustless Systems**: Creating a platform where presence integrity is guaranteed by code, eliminating the need for manual verification.

## Key Features

### 1. **Immutable Attendance Recording**
- Record a participant's presence with a single function call.
- Securely stores the participant's `nim` (Nomor Induk Mahasiswa / Student ID) and `name`.
- Automated unique ID generation for each attendance record.

### 2. **Efficient Data Retrieval**
- Fetch all stored attendance records in a single call.
- Structured data representation (`Vec<AttendanceRecord>`) for easy auditing by event organizers or lecturers.

### 3. **Secure Record Management**
- Capability to remove specific invalid attendance records using their unique IDs.
- Immediate update of the ledger state after modification.

### 4. **Transparency and Security**
- View all attendance activities transparently on the Stellar network.
- Protected against unauthorized modifications through blockchain consensus.

## Future Scope

### Short-Term Enhancements
1. **Wallet Address Integration**: Upgrade the `nim` parameter to accept Soroban `Address` types, mapping physical students to actual cryptographic wallets.
2. **Timestamping**: Add automated blockchain timestamps to record exactly *when* a student submitted their presence.
3. **Event Structuring**: Allow the contract to manage multiple distinct events/classes simultaneously.

### Medium-Term Development
4. **POAP Minting**: Automatically mint and send a Non-Fungible Token (NFT) badge to the attendee's wallet upon successful presence verification.
5. **Geolocation/Backend Validation**: Integrate an off-chain oracle or backend server to verify the user's GPS location or scan a dynamic QR code before allowing the smart contract to be invoked.

### Long-Term Vision
6. **University-Wide Integration**: Adopt the system as the standard decentralized presence protocol across faculties.
7. **Cross-Chain Synchronization**: Extend presence credential storage to multiple networks.

---

## Technical Requirements

- Soroban SDK
- Rust programming language
- Stellar blockchain network (Testnet/Futurenet)
- **Soroban Studio** (Recommended for cloud compilation and direct interaction)

## Getting Started (Via Soroban Studio)

Since this project focuses on the smart contract architecture, you can easily deploy and interact with it without a frontend:

1. Open this project in **Soroban Studio**.
2. **Build** the contract using the provided tools in the studio.
3. **Deploy** the contract to the Stellar **Testnet**.
4. Use the **Interact** panel to test the following main functions:
   - `record_attendance(nim: String, name: String)` - Add a new student presence to the blockchain.
   - `get_attendance()` - Retrieve the list of all recorded attendances.
   - `remove_attendance(id: u64)` - Delete a specific record by its generated ID.

---

**Stellar Attendance System** - Securing Your Credentials on the Blockchain