import sqlite3
import string

letters = set(string.ascii_letters)

con = sqlite3.connect('wordle.db')

def insert_word(words):
    cur = con.cursor()
    cur.executemany(
        '''INSERT INTO words (word, len) VALUES (?, ?)''',
        words
    )
    con.commit()

def create_table():
    cur = con.cursor()
    cur.execute('''
    CREATE TABLE words (
        word TEXT PRIMARY KEY,
        len INT
    )
    ''')
    con.commit()

create_table()


words_file = open('/usr/share/dict/american-english')

words = set()
for word in words_file:
    word = word.strip()
    for l in word:
        if l not in letters:
            break
    else:
        words.add(word.lower())

print(len(words))
insert_lists = []
for i, w in enumerate(words):
    insert_lists.append((w, len(w)))
    
insert_word(insert_lists)

con.close()
