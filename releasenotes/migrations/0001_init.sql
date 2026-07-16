-- 0001_init.sql — Schema + Seed (generiert aus DevOps-Stand)

CREATE TABLE versions (
    workitem_id BIGINT PRIMARY KEY,          -- System.Id der Funktion
    version     TEXT   NOT NULL UNIQUE       -- Funktions-Titel, z.B. '9.0.2502.9'
);

CREATE TABLE abkuendigungen (
    workitem_id BIGINT PRIMARY KEY,          -- System.Id der Abkündigung
    titel       TEXT   NOT NULL,             -- relnoTitel
    version_id  BIGINT REFERENCES versions(workitem_id)
);

-- Seed: veröffentlichte Versionen (36 gelistete + 2 ältere, an die Abkündigungen hängen)
INSERT INTO versions (workitem_id, version) VALUES
    (32793, '8.3.2210.20'),
    (33001, '8.3.2211.9'),
    (33038, '8.3.2211.30'),
    (33240, '8.3.2212.23'),
    (33359, '8.3.2302.17'),
    (33541, '8.3.2303.31'),
    (33732, '8.3.2304.28'),
    (33801, '8.3.2309.1'),
    (33828, '8.3.2305.26'),
    (33878, '8.3.2306.9'),
    (33926, '8.3.2306.23'),
    (33952, '8.3.2307.7'),
    (34053, '8.3.2308.4'),
    (34143, '8.3.2308.18'),
    (34212, '9.0.2401.1'),
    (34418, '8.3.2310.27'),
    (34465, '8.3.2311.10'),
    (34514, '8.3.2312.8'),
    (34585, '8.3.2312.22'),
    (34954, '9.0.2402.1'),
    (34955, '9.0.2401.2'),
    (35164, '9.0.2401.3'),
    (35300, '9.0.2401.4'),
    (35674, '9.0.2402.2'),
    (35676, '9.0.2402.3'),
    (35845, '9.0.2402.4'),
    (36261, '9.0.2402.8'),
    (36318, '9.0.2402.10'),
    (36664, '9.0.2502.5'),
    (37113, '9.0.2501.5'),
    (37228, '9.0.2501.6'),
    (37501, '9.0.2501.8'),
    (38106, '9.0.2502.6'),
    (38376, '9.0.2502.7'),
    (38555, '9.0.2502.8'),
    (38940, '9.0.2502.9'),
    (32648, '8.3.2207.18'),
    (32869, '8.3.2204.11');

-- Seed: Abkündigungen (workitem_id, relnoTitel, version_id) — ohne Freitext
INSERT INTO abkuendigungen (workitem_id, titel, version_id) VALUES
    (31998, 'Abkündigung: Tammo MAPI', 32869),
    (32038, 'Abkündigung: A1extern.dll', 32648),
    (32039, 'Abkündigung: JRCON-Online Waage', 32648),
    (32040, 'Abkündigung: JRCON-Belegversand', 32648),
    (32041, 'Abkündigung: JRCON/GS MDE – Einzelhandelslösung MOBILAR (OFFLINE)', 32648),
    (32042, 'Abkündigung: GS-Kommissionierlösung (Offline)', 32648),
    (32043, 'Abkündigung: EDI – Spezielle Lösungen', 32648),
    (32044, 'Abkündigung: Wechselbuchhaltung', 32648),
    (32045, 'Abkündigung: Mehrmandantensystem', 32648),
    (33065, 'Preiskalkulation', 33038),
    (33581, 'OLE Steuerparameter', 33541),
    (34636, 'Excelimport von xls-Dateien', 34212),
    (36368, 'Abkündigung: Infocenter', 37113),
    (36624, 'Abkündigung: Callback Dialog', 36664);
