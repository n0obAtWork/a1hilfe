# Tabellen des Vorgangsimports

<!-- source: https://amic.de/hilfe/_vimp_tabellen.htm -->

Folgende Relationen müssen für das Importieren von Vorgängen befüllt werden.

| Pflicht Relationen | |
| --- | --- |
| [ImportVorgStamm](../vorgangsimport_anwendung/funktionen_des_vimp_pflegers.md#Tab_ImpoprtVorgStamm) | In dieser Relation müssen alle Daten eingefügt welche für den Vorgangskopf benötigt werden.  
z.B. Kundennummer, Vorgangsklasse, Vorgangsunterklasse |
| [ImportVorgPosition](../vorgangsimport_anwendung/funktionen_des_vimp_pflegers.md#Tab_ImportVorgPosition)  
 | In dieser Relation müssen alle Daten gespeichert die auf Positionsebene benötigt werden.  
z.B. Artikelnummer, Menge, Mengeinheit |

| Optionale Relationen | |
| --- | --- |
| [ImportVorgTextPosition](../vorgangsimport_anwendung/funktionen_des_vimp_pflegers.md#Tab_ImportVorgTextPosition) | In dieser Relation können Texte zu einer Position gespeichert werden.  
Mit dem Feld Textposition kann bestimmt werden, wenn es sich um eine Textposition handelt, ob der Text vor oder nach einer Position angezeigt werden soll.  
• 0 Vor der Position  
• 1 Nach der Position  
0 ist der Defaultwert  
Die Reihenfolge bestimmt der Zeilenzähler.  
Folgende Texttypen können gesetzt werden  
• 0 Positionstext  
0 ist der Defaultwert des Feldes.  
Die Textlänge ist auf 255 Zeichen Begrenzt. |
| ImportVorgPositionAddon | Veraltet !!! Bitte [ImportVorgPosiAddOn](./importvorgposiaddon.md) verwenden!  
In dieser Relation können Daten für die Addonfelder gespeichert werden. |
| [ImportVorgPosiAddOn](./importvorgposiaddon.md) | In dieser Relation werden Daten gespeichert, die später in der Tabelle WarenbewegungAddon zur Position hinterlegt werden sollen.  
Der Name des gegebenen AddOn-Feldes muss mit dem Feldnamen in der Tabelle übereinstimmen, da sonst keine Daten gespeichert werden können. |
| [ImportVorgPositionPartie](./importvorgpositionpartie.md) | In dieser Relation werden Informationen der Partie(n) einer Position abgelegt. Eine Partie, die hier eingetragen ist, jedoch im System noch nicht existiert, wird angelegt werden. |
| [ImportVorgStammZusatzTexte](../vorgangsimport_anwendung/funktionen_des_vimp_pflegers.md#Tab_ImportVorgStammZusatzTexte) | In dieser Relation können Zusatztexte zum Vorgang gespeichert werden. Diese lassen sich dann später mit SQLK auf bestimmten Dokumenten andrucken. Diese Texte sind nicht zu verwechseln mit den Vorgangstexten! |
| [ImportVorgStammAddon](./importvorgstammaddon.md) | Hier werden Addon-Daten zum Vorgang hinterlegt. |
| [ImportVorgStammUFLD](../vorgangsimport_anwendung/funktionen_des_vimp_pflegers.md#Tab_ImportVorgStammUFLD) | Hier werden die Werte von UFLD-Feldern mit deren ID hinterlegt, die für den zu importierenden Vorgang gesetzt werden sollen. |
| [ImportVorgPositionLVS](../vorgangsimport_anwendung/funktionen_des_vimp_pflegers.md#Tab_ImportVorgPositionLVS) | Diese Relation beherbergt Informationen zu LVS-Ladeträgern, die zu dieser Position gehören.  
 |
| [ImportVorgStammZuAb](../vorgangsimport_anwendung/funktionen_des_vimp_pflegers.md#Tab_ImpoprtVorgStammZuAb) | Link zwischen ImportVorgStamm und ImportVorgZuAbDef |
| [ImportVorgPositionZuAb](../vorgangsimport_anwendung/funktionen_des_vimp_pflegers.md#Tab_ImpoprtVorgPositionZuAb) | Link zwischen ImportVorgPosition und ImportVorgZuAbDef |
| [ImportVorgZuAbDef](./importvorgzuabdef.md) | Definition eines Zu/Abschlags, eines Rabattes oder einer Fracht |

Referentielle Integrität

ImportVorgStamm

In der Tabelle ImportVorgStamm, sowie den untergeordneten Tabellen, werden durch die Fremdschlüsselbeziehungen („Foreign Keys“) über die Spalte IVS_Guid, automatisch alle Datenbestände aus den folgenden Tabellen gelöscht.

\- ImportVorgStammUFLD

\- ImportVorgStammAddon

\- ImportVorgStammZusatzTexte

In der Tabelle ImportVorgPosition wird bei einer Löschung eines ImportVorgStamm-Datensatzes, die entsprechende IVS_Guid auf den Wert NULL gesetzt, der ImportVorgPosition-Datensatz bleibt aber erhalten.

Beim Update der IVS_Guid werden abhängige Fremdschlüssel kaskadierend aktualisiert.

ImportVorgPosition

In der Tabelle ImportVorgPosition, sowie den untergeordneten Tabellen, werden durch die Fremdschlüsselbeziehungen über die Spalte IVP_Guid, automatisch alle referenzierten Datenbestände gelöscht.

\- ImportVorgPositionPartie

\- ImportVorgPosiAddon

In den folgenden Tabellen wird beim Szenario einer Löschung eines ImportVorgPosition-Datensatzes die entsprechende IVP_Guid auf den Wert NULL gesetzt, die Datensätze bleiben aber erhalten.

\- ImportVorgTextPosition

\- ImportVorgScannung

\- ImportVorgPositionLVS

\- ImportVorgPositionAddon

Beim Update der IVP_Guid werden abhängige Fremdschlüssel kaskadierend aktualisiert.

Trigger auf Tabelle ImportVorgPosition

Beim Einfügen einer ImportVorgPosition wird über einen "before-insert" Trigger geprüft, ob die ImportVorgPosition zu einem Datensatz in der Tabelle ImportVorgStamm passt.

Findet der Trigger einen entsprechenden Eintrag, wird die entsprechende IVS_Guid in dem ImportVorgPosition-Datensatz eingetragen, ansonsten wird der Wert auf NULL gesetzt.

Um einen A.eins Vorgang mit dem Vorgangsimport in das A.eins System einzuspielen müssen folgende Regeln beachtet werden und mindestens folgende Felder gefüllt werden. Das Einspielende System muss dafür sorgen, dass alle Felder mit A.eins konformen Daten versehen werden.

<p class="siehe-auch">Siehe auch:</p>

- [Importvorgstamm](./importvorgstamm.md)
- [ImportVorgStammUFLD](./importvorgstammufld.md)
- [ImportVorgStammAddon](./importvorgstammaddon.md)
- [ImportVorgStammZuAb](./importvorgstammzuab.md)
- [ImportVorgStammZusatzTexte](./importvorgstammzusatztexte.md)
- [ImportVorgPosition](./importvorgposition.md)
- [ImportVorgPosiAddon](./importvorgposiaddon.md)
- [ImportVorgTextPosition](./importvorgtextposition.md)
- [ImportVorgPositionLVS](./importvorgpositionlvs.md)
- [ImportVorgPositionPartie](./importvorgpositionpartie.md)
- [ImportVorgPositionZuAb](./importvorgpositionzuab.md)
- [ImportVorgZuAbDef](./importvorgzuabdef.md)
