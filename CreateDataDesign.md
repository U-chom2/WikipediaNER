# NER用学習データ作成手順

1. Wikidataの加工
   1. ダンプversion選定（竹内研では固定になる）
   2. ２種類のダンプが必要
      - すべてのリレーションが乗っているrdf
      - wikidataのタイトルとwikipediaのタイトルが付属するrdf(wikipediaであるなら不必要)
   3. ダンプはでかいのでenまたはjaだけに圧縮

1. wikipediaの加工
   1. ダンプのversion選定
   2. wikidataとwikipediaのリンクが記載されたダンプがあるならこちらを優先
   3. ダンプをDBに流し込み（解凍の必要なし）DB_1
   4. 形態素解析で解析器ごとにテーブルを用意（id、onehot_id、語、品詞、NE、wikidataのP31（この時点では空）、etc）でDBに登録 DB_2

1. wikipediaのタイトルとwikidataのP31のdicを作成
1. DB_2に流し込み
