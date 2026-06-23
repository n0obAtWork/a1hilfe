# OSQL

<!-- source: https://amic.de/hilfe/osql.htm -->

Alle folgenden Befehle sind nicht SQL-Standard, sondern unter Aeins implementiert, um kurzfristige systemnahe Operationen zu vereinfachen bzw. um Skripte für einmalige Prozesse zu schreiben. Weiterhin können JPL und Pascalskripte von hier aus gestartet werden.

<p class="just-emphasize">Handhabung</p>

In dem Eingabefenster links oben können SQL-Befehle eingegeben und mit **F9** ausgeführt werden. Das Ergebnis erscheint dann in dem Feld darunter. Will man wieder auf den vorherigen Befehl zugreifen, so kann man mit **Strg+Pfeil nach Oben** bzw. **Strg+Pfeil nach Unten** in den Kommandos blättern. Mit **Strg+F3** kann man eine F3-Auswahl öffenen in der alle bisher ausgeführten Befehle angezeigt werden. Man kann dort auch auf Befehle anderer Benutzer zugreifen.

Eine weitere hilfreiche Taste ist **Tab** bzw. **Shift+Tab**. Mit ihr kann zwischen den Tabellen, Views, Prozeduren und Triggern – dies wird je nach Kommando bestimmt – weitergeblättert werden. Bei **Tab** wird das in alphabetischer Reihenfolge nächste Datenbankobjekt angezeigt, bei **Shift+Tab** das vorherige. Gibt man als z.B.

```text
show view
AMIC_V_D
```

und drückt **Tab**, dann wird das Kommando automatisch auf

```text
show view
AMIC_V_DATEVSTAMM
```

erweitert.

Will man ein Datenbankobjekt eines bestimmten Anwenders sehen, so kann man dessen Kürzel vorwegstellen (In diesem Fall Test Bediener: TB). Dies wird auch von **Tab** und **Shift+Tab** berücksichtigt:

```text
show VIEW
TB.MandantBitmap
```

<p class="siehe-auch">Siehe auch:</p>

- [Optionen (F10)](./optionen_f10.md)
- [Sammlung der Kommandos](./sammlung_der_kommandos/index.md)
