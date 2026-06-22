# Sperren

<!-- source: https://amic.de/hilfe/frz_sperren.htm -->

Manche Felder auf der Registerkarte Sperren sind von der gewählten Vorgangsklasse abhängig.

Vorbelegungen

| Maskenfeld | Vorbelegung | Bedeutung |
| --- | --- | --- |
| Vorbel. Fibu-Sperre | Nein | Beleg wird nach der Erzeugung gegen den Übertrag in die Fibu gesperrt. Beleg muss manuell Freigegeben werden. |
| Vorbel. Umw-Sperre | Nein | Der Beleg kann nach der Erzeugung nicht Umgewandelt werden. Beleg muss manuell Freigegeben werden. |
| Vorbel. RAB-Sperre | Nein | Der Beleg kann nach der Erzeugung nicht in das rechnungsausgangsbuch übernommen werden. Beleg muss manuell Freigegeben werden. |
| Vorbel. Filia-Sperre | Nein | Wirkungslos |

Behandlungen exportierter Vorgänge

Hier kann eingestellt werden, ob bei einem Beleg der das Kennzeichen V_StatusExport im Vorgangstamm ungleich 0 hat folgenden Funktionen zu gelassen werden.

| Maskenfeld | Vorbelegung |
| --- | --- |
| Löschen erlaubt | Nein |
| Stornieren erlaubt | Nein |
| Umwandeln erlaubt | Nein |
| Korrigieren erlaubt | Nein |

Kennzeichen bei Erfassungen

| Maskenfeld | Vorbelegung | Bedeutung |
| --- | --- | --- |
| Persönliches Kennzeichen abfragen | Nein | Hier kann eingestellt werden, ob der Bediener beim Erfassen eines Vorgangs sein Persönliches Kennzeichen eingeben muss. Das persönliche Kennzeichen wird im Bedienerstamm festgelegt. Der Vorgang wird dann unter dem Bediener erfasst, der sein Kennzeichen angegeben hat. Dies kann zur Kontrolle eingesetzt werden, wenn mehrere Bediener sich ein Arbeitsplatz teilen. |
| EPA hebt Kennzeichenabfrage auf | Nein | Hier kann eingestellt werden dass die Kennzeichenabfrage per EPA für eine bestimmte Bedienerklasse ausgestellt werden kann. |

Sonstige Sperren

| Maskenfeld | Vorbelegung | Bedeutung |
| --- | --- | --- |
| Bearbeitungssperre auch bei Folgebelegen  
 | Nein | Die Sperre legt fest, ob Folgebelege aus einem Quellbeleg dieser Vorgangsklasse geändert oder gelöscht werden können, wenn der Quellbeleg eine Bearbeitungssperre gesetzt hat.  
 |
| Korrektur mit Doppelklick verbieten | Nein | Die Sperre legt fest, ob ein Doppelklick auf eine Positionszeile den Bearbeitungsmodus aufruft. Dies gilt nicht für Rohwarenbelege. |

Ladescheine

| Maskenfeld | Vorbelegung | Bedeutung |
| --- | --- | --- |
| Sperren und nicht Stornieren | Nein | Hier kann eingestellt werden, ob nach der Umwandlung eines Ladescheins zu einem Lieferschein oder Rechnung der Ladeschein storniert werden soll oder ob der Ladeschein gegen Weiterverarbeitung geschützt wird. |

Teildisposition

| Maskenfeld | Vorbelegung | Bedeutung |
| --- | --- | --- |
| Mengenüberziehung zulassen bei Teildisposition aus dieser Unterklasse | Nein | Hier kann eingestellt werden, ob bei der Erzeugung von Vorgangspositionen per Teildisposition aus Positionen eines anderen Vorgangs dieser Unterklasse die aus der Quellposition abzubuchende Menge größer als die noch vorhandene Restmenge sein darf. |
| Mengenüberziehung erledigt Beleg | Ja | Wird hier „Nein“ angewählt, wird der Beleg bei Überziehung der Quell-Menge nicht als vollständig gekennzeichnet, so dass eine nachträgliche Änderung mittels [AUK] möglich wird.  
Ein „Ja“ setzt den Beleg bei Überziehung auf erledigt |
| Teildisponierte Position editierbar | Nein | Wurde eine Position im Quellbeleg teildisponiert, so ist sie im Standard nicht mehr editierbar. Dieser Schalter erlaubt das nachträgliche Editieren einer teildisponierten Position |

**Achtung!**

Diese Einstellungen können in Abhängigkeit von [SPA 1053](../../firmenstamm/steuerparameter/optionen_warenwirtschaft/lesen_der_teildispositionseinstellungen_aus_richtiger_unterk.md) durch die Einstellungen der Unterklasse 0 überschrieben werden.

UlsatzSteuerid

| Maskenfeld | Vorbelegung | Bedeutung |
| --- | --- | --- |
| druckbar  
 | Wenn UStId geprüft | Ist eine UStId nicht erfolgreich geprüft worden, so kann ein Beleg nicht gedruckt werden, wenn Intrastat lizensiert ist und der [Steuerparameter 1062 – UmsatzSteuerId im Vorgang](../../firmenstamm/steuerparameter/vorgangsbearbeitung_allg/ustid_pruefung_im_vorgang_spa_1062.md) gesetzt ist |
| An Fibu übertragbar | Wenn UStId geprüft | .Ist eine UStId nicht erfolgreich geprüft worden, so kann ein Beleg nicht an die FiBu übertragen werden, wenn Intrastat lizensiert ist und der [Steuerparameter 1062 – UmsatzSteuerId im Vorgang](../../firmenstamm/steuerparameter/vorgangsbearbeitung_allg/ustid_pruefung_im_vorgang_spa_1062.md) gesetzt ist |

Hier gibt es jeweils drei Abstufungen

• „Wenn UStId geprüft“

• „wenn eingetragen“

• „immer“
