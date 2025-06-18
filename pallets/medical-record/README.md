# Medical Record Pallet

A comprehensive medical record management system built with Polkadot SDK, designed to handle patient information, clinical tests, and disease progression tracking.

## Overview

This pallet provides three main entities for medical record management:

1. **Patient Information (TT_Bệnh nhân)**: Basic patient demographics and contact information
2. **Clinical Tests (TT_Cận lâm sàng)**: Laboratory and diagnostic test records
3. **Disease Progression (TT_Diễn biến bệnh)**: Visit records, symptoms, diagnosis, and treatment tracking

## Features

### CRUD Operations for all entities:

- **Create**: Add new patients, clinical tests, and disease progression records
- **Read**: Query existing records (via storage getters)
- **Update**: Modify existing records
- **Delete**: Remove records from storage

### Data Structures

#### Patient Information

- Patient ID (auto-generated)
- Name, Date of Birth, Gender
- Address, Phone, Emergency Contact
- Creation timestamp

#### Clinical Tests

- Test ID (auto-generated)
- Patient ID (reference)
- Doctor ID (who ordered the test)
- Test Type, Date, Results, Notes
- Creation timestamp

#### Disease Progression

- Progression ID (auto-generated)
- Patient ID (reference)
- Doctor ID (attending physician)
- Visit Date, Symptoms, Diagnosis
- Treatment, Prescription, Next Appointment
- Creation timestamp

## Usage

### Creating a Patient

```rust
MedicalRecord::create_patient(
    origin,
    patient_name,
    date_of_birth,
    gender,
    address,
    phone,
    emergency_contact
)
```

### Adding Clinical Test

```rust
MedicalRecord::create_clinical_test(
    origin,
    patient_id,
    test_type,
    test_date,
    result,
    notes
)
```

### Recording Disease Progression

```rust
MedicalRecord::create_disease_progression(
    origin,
    patient_id,
    visit_date,
    symptoms,
    diagnosis,
    treatment,
    prescription,
    next_appointment
)
```

## Security & Privacy

- All operations require signed transactions
- Patient data validation on creation
- Referential integrity checks (patient must exist before adding tests/progressions)
- Immutable creation timestamps for audit trails

License: MIT-0
