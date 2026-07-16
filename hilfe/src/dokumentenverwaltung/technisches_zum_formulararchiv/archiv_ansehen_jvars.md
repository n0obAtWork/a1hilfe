# Archiv ansehen (JVARS)

<!-- source: https://amic.de/hilfe/archivansehenjvars.htm -->

Beim Ausführen einer Archiv-Ansicht werden Datenermittlungen gemäß der in der zugehörigen [Archiv-Ansichts-Definition](../archiv_ansehen/archiv_ansicht_definition/index.md) und in den [Archiv-Ansichten Details](../archiv_ansehen/archiv_ansichten_definieren/archiv_ansichten_details.md) hinterlegten Regeln und Vorschriften erhoben. Die so ermittelten Daten werden in speziellen JVARS hinterlegt. Die Kenntnis dieser JVARS erlaubt es diese z.B. in [Ableitungen](../archiv_ansehen/ansichten_allgemein/die_auswahlliste_formulararchiv_anzeige/index.md#ueb_Archiv_Ableitung) zu nutzen.

| **Parameter-JVars des Owners 5001** | **Informatorisch favv_id aus fa_view_vars** |
| --- | --- |
| REFERENZ | 0 |
| KUNDNUMMER | 1 |
| ZW1 | 2 |
| ZW2 | 3 |
| ZW3 | 4 |
| ZW4 | 5 |
| ZW5 | 6 |
| JahrBeginn | 100 |
| JahrEnde | 101 |
| Belegtyptext | 102 |
| Belegklasse | 103 |
| Belegnummer | 104 |
| GRUPPE | 200 |
| LINIE | 201 |
| freies ODER | 300 |
| freies UND | 301 |
| freies JOIN | 302 |

Die obigen JVARS sind dann belegt, wenn sie in den [Archiv-Ansichten Details](../archiv_ansehen/archiv_ansichten_definieren/archiv_ansichten_details.md) angegeben wurden. Zudem existiert zusätzlich eine paarige JVAR mit fast gleichem Namen (um ein Ausrufezeichen erweitert). Diese JVAR hat 0 oder 1 als Inhalt, je nachdem ob es sich um einen „selektionswirksamen“ Parameter handelt. Selektionswirksame JVARS werden zur Konstruktion des Where-Statements herangezogen.

| **Sql-JVars des Owners 5001** |
| --- |
| FAA_JOIN |
| FAA_AUSWAHL |

Im Standard sowie im „Vorschau-Modus“ der Archiv-Anzeige ist die zugeordnete Variante im Ansichts-Profil hinsichtlich der SQL-Statement-Gewinnung maßgeblich. (siehe [Hauptmenü Administration/Archiv/Archiv-Ansichten](../archiv_ansehen/archiv_ansicht_definition/index.md#ueb_Archiv_Variante) , Auslieferungsvariante ist „fa_anzeige“)

In der Auslieferungsvariante ist der Zusammenhang mit den Sql-JVars des Owners 5001 zu erkennen:

```sql
SQL
 select :FIELDS from formulararchiv fa
 :!JVARS_5001_FAA_JOIN
 where ( 1=1 )
 :!JVARS_5001_FAA_AUSWAHL
 order by fa.FA_Druckdatum desc
```

Somit werden mit Hilfe der [Archiv-Ansichts-Definition](../archiv_ansehen/archiv_ansicht_definition/index.md) die Inhalte der obigen JVARS ermittelt.

| **Weitere JVars des Owners 5001 zwecks Where-Ermittlung.** | |
| --- | --- |
| JVAR_FAARCHIV_VIEW_AND | Summierungsergebnisse der „freies UND“ |
| JVAR_FAARCHIV_VIEW_OR | Summierungsergebnisse der „freies ODER“ |

| **Administrative Sql-JVars des Owners 5001** | |
| --- | --- |
| JVAR_FAARCHIV_VIEW_FAVP_ANWID | Technische ID der [Archiv-Ansichts-Definition](../archiv_ansehen/archiv_ansicht_definition/index.md) |
| JVAR_FAARCHIV_VIEW_FAVP_BESITZER | Besitzer |
| JVAR_FAARCHIV_VIEW_UMGEBUNG | Textuelle Aufbereitung: Profilname, Id und Besitzer<br>z.B. Profil: AMIC_KUNDE, Anischts-ID: 32, Besizer: 0 |
| JVAR_FAARCHIV_VIEW_START | ["Durchstart"](../archiv_ansehen/archiv_ansicht_definition/index.md#ueb_Archiv_Durchstart) der Archiv-Ansichts-Definition |

| **Temporäre Sql-JVars des Owners 5001** |
| --- |
| JVAR_FAARCHIV_VIEW_WERT |
| JVAR_FAARCHIV_VIEW_ADD |

Diese JVARS dienen ausschließlich der internen Verwendung.

| **Sql-JVars des Owners 5000** | [**Datenherkunft aus 5001**](./archiv_ansehen_jvars.md#ueb_Archiv_5001) |
| --- | --- |
| JVARS_FA_KTX_REF | REFERENZ |
| JVARS_FA_KTX_KU | KUNDNUMMER |
| JVARS_FA_KTX_BKL | Belegklasse |
| JVARS_FA_KTX_BNR | Belegnummer |

Diese JVARS werden bei der Ermittlung der [JVARS mit Owner 5001](./archiv_ansehen_jvars.md#ueb_Archiv_5001) befüllt und dienen als Vorbelegung für [Archiv hinzufügen](../archiv_dokumente_hinzufuegen.md).
