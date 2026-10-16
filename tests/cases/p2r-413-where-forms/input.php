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

Book::create(['title' => 'Dune', 'shelf' => 1, 'note' => 'sand', 'rank' => 2]);
Book::create(['title' => 'Emma', 'shelf' => 1, 'rank' => null]);
Book::create(['title' => 'Dune', 'shelf' => 2, 'note' => 'again', 'rank' => 1]);
// The first match in creation order; a string compares byte for byte.
echo titled('Dune'), ' ', titled('dune'), "\n";
echo noted('again'), ' ', noted('none'), "\n";
echo onShelf(1, null), ' ', onShelf(1, 2), ' ', onShelf(2, 2), "\n";
$wanted = 'Emma';
$book = Book::query()->where('title', $wanted)->where('note', null)->first();
echo $book === null ? 'none' : $book->title, "\n";
