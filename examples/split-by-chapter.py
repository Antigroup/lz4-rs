
book = open('moby.txt').read()
print(len(book))

chapter_number = 1

while True:
    begin = book.find('CHAPTER ' + str(chapter_number))
    if begin == -1:
        break
    end = book.find('CHAPTER ' + str(chapter_number + 1))

    chapter = book[begin:end]
    with open('chapters/' + str(chapter_number) + '.txt', mode='w') as out:
        out.write(chapter)

    chapter_number += 1

