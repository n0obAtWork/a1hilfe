# Übergabe an  die RFS-Schnittstelle

<!-- source: https://amic.de/hilfe/bergabeandierfsschnittstelle.htm -->

### Übergabe an die RFS-Schnittstelle

Bei eingeschalteter RFS-Schnittstelle (die Schnittstelle wird durch einen Lizenzparameter frei geschaltet ) wird die Belegübergabe im Rahmen der im Aeins üblichen Übergabe an die FIBU realisiert. Organisatorische Abhängigkeiten ( Belegstatus, Zusammenhang mit dem Mandentenserver etc ) und Eingrenzungsmöglichkeit des FIBU-Übertrages entsprechen komplett der üblichen Aeins –Logik. Ein Beleg befindet sich genau dann in der RFS-Schnitstelle, wenn er als ‚IN FIBU’ gekennzeichnet ist. So werden z.B. Belege, die nicht übertragen werden konnten ( fehlende Steuerzuordnungen / Erlöskennziffernprobleme etc ) in den für den ‚normalen’ Fibuübertrag geführten Protokollen eingetragen ( z.B. Direktsprung JOUR ). Das RFS System stellt diese Belege erst dann in die RFS-Schnittstelle ein, wenn die sonst üblichen Vorraussetzungen für einen ordnungsgemäßen Fibu-Übertrag erfüllt sind. Fehlermeldungen bezüglich inkonsistenter Einstellung der RFS Parameter hingegen werden in einer gesonderten Protokolldatei notiert ( siehe RFS Voreinstellungen RFSV ).
