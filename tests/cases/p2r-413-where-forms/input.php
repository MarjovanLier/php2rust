<?php

declare(strict_types=1);

namespace App;

use Illuminate\Database\Eloquent\Model;

final class Book extends Model
{
    protected $fillable = ['title', 'shelf', 'note', 'rank'];

    public $timestamps = false;
}

function titled(string $title): string
{
    $book = Book::query()->where('title', $title)->first();
    return $book === null ? 'none' : $book->title;
}

function noted(string $note): string
{
    $book = Book::query()->where('note', $note)->first();
    return $book === null ? 'none' : $book->title;
}

function onShelf(int $shelf, ?int $rank): string
{
    $book = $rank === null
        ? Book::query()->where('shelf', $shelf)->where('rank', null)->first()
        : Book::query()->where('shelf', $shelf)->where('rank', $rank)->first();
    return $book === null ? 'none' : $book->title;
}

// The closure's own names step aside for variables of the same Rust name.
function sameTitle(Book $row): int
{
    $book = Book::query()->where('title', $row->title)->where('shelf', 2)->first();
    return $book === null ? 0 : $book->id;
}

function notedAs(string $text): string
{
    $book = Book::query()->where('note', $text)->first();
    return $book === null ? 'none' : $book->title;
}

// The tool knows the value of $row here, and the closure's name still steps aside.
function onShelfTwo(): string
{
    $row = 2;
    $book = Book::query()->where('shelf', $row)->first();
    return $book === null ? 'none' : $book->title;
}

Book::create(['title' => 'Dune', 'shelf' => 1, 'note' => 'sand', 'rank' => 2]);
Book::create(['title' => 'Emma', 'shelf' => 1, 'rank' => null]);
Book::create(['title' => 'Dune', 'shelf' => 2, 'note' => 'again', 'rank' => 1]);
// The first match in creation order; a string compares byte for byte.
echo titled('Dune'), ' ', titled('dune'), "\n";
echo noted('again'), ' ', noted('none'), "\n";
echo onShelf(1, null), ' ', onShelf(1, 2), ' ', onShelf(2, 2), "\n";
$first = Book::find(1);
if ($first !== null) {
    echo sameTitle($first), ' ', notedAs('sand'), "\n";
}
echo onShelfTwo(), "\n";
$below = Book::query()->where('shelf', -1)->first();
echo $below === null ? 'none' : $below->title, "\n";
$wanted = 'Emma';
$book = Book::query()->where('title', $wanted)->where('note', null)->first();
echo $book === null ? 'none' : $book->title, "\n";
