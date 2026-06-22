# Kommandozeile

<!-- source: https://amic.de/hilfe/_LiLaKommandozeile.htm -->

Man kann einen auf einer Prozedur basierenden AMIC Etikettendruck Report auch von der Kommandozeile aus starten. Der Aufruf muss dann wie folgt aussehen:

```text
aeins
welcome entw pda=lila ID1=LILAID ID2=1 ProcedureCall="...(...)"
[Printerprofil=...]
```

| Parameter | Beschreibung |
| --- | --- |
| PDA | Weist A.eins an, die folgende Maske direkt zu starten. Wird als Maske ETIDR angegeben, so wird versucht, den unter ID1 ausgewählte Report direkt zu drucken.<br> |
| ID1 | Funktionsident des zu startenden Reports<br> |
| ID2 | Besitzer des Reports. Bei privaten Reporten immer 1<br> |
| PrinterConfig<br>*(veraltet)* | Dieser Parameter wird nicht mehr unterstütz. Wird er noch verwendet, so führt er zu einem Eintrag im Fehlerprotokoll. Der Report/ das Etikett wird trotzdem gedruckt. |
| PrinterProfil<br>(Optional) | Löst den Parameter PrinterConfig ab. Man gibt hier einfach das [Profil](./definition_in_a_eins.md#Druckerprofile) an, dass man bei der Bearbeitung der Definitionen in A.eins erstellt hat.<br> |
| ProcedureCall | Dieser Parameter gibt an, was überhaupt gedruckt werden soll. Das Format muss so sein, wie bei Prozeduren der [Aufruf für bearbeiten](./definition_in_a_eins.md#AufrufZumBearbeiten) eingetragen wurde.<br> |
| BelegVId<br>*(Optional)* | Mit diesem Parameter kann die „V_ID“ eines Vorgangs übergeben werden. Der Report wird dann erst gedruckt, wenn die Verarbeitung des Beleges durch den Mandantenserver erfolgt ist.<br>Der Parameter darf nicht verwendet werden, wenn der Aufruf durch den Mandantenserver erfolgt. Denn Mandantenserver würde sich selbst blockieren.<br> |

Beispiel:

```text
Aeins
welcome entw PDA=LILA ID1=FUNCTEST ID2 =1 PrinterProfil=KyoseraSchacht2
Procedurecall=“DBProc(451)“
```

Treten hierbei Fehler auf, so werden diese in Fehlerprotokoll (Direktsprung [FEHLP]) geschrieben.
