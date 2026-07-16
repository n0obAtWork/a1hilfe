# Anlegen der Relationen

<!-- source: https://amic.de/hilfe/anlegenderrelationen.htm -->

create table ScriptParam (

ScriptPBedKorr INTEGER NOT NULL DEFAULT 0,

ScriptPBesitzer smallint NOT NULL DEFAULT 0,

ScriptPBezeich char(50) NOT NULL,

ScriptPId char(20) NOT NULL UNIQUE,

ScriptSystem smallint NOT NULL DEFAULT 0,

primary key (ScriptPId )

);

create table tScriptParamPar (

ScriptPId char(20) NOT NULL,

ScriptPPAktiv smallint NOT NULL DEFAULT 1,

ScriptPPBedKorr INTEGER NOT NULL DEFAULT 0,

ScriptPPBezeich char(50) NOT NULL,

ScriptPPId char(30) NOT NULL,

ScriptPPTyp smallint NOT NULL DEFAULT 0,

ScriptPPWert1 char(50) NOT NULL DEFAULT '',

ScriptPPWert2 char(50) NOT NULL DEFAULT '',

ScriptPPWert3 char(50) NOT NULL DEFAULT '',

ScriptSystem smallint NOT NULL DEFAULT 0,

primary key (ScriptPId ,ScriptPPId )

);
