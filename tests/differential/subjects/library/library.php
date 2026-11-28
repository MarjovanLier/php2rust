<?php

declare(strict_types=1);

namespace Library;

use Illuminate\Database\Eloquent\Collection;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\ModelNotFoundException;
use Illuminate\Database\MultipleRecordsFoundException;

// Books on shelves, written by hand for the harness: creation, the where forms, the reads
// of a collection, and Laravel's two exceptions with their messages.
final class Book extends Model
{
    protected $fillable = ['title', 'shelf', 'note', 'rank'];

    public $timestamps = false;
}

function shelve(string $title, int $shelf, ?string $note, ?int $rank): int
{
    $book = Book::create(['title' => $title, 'shelf' => $shelf, 'note' => $note, 'rank' => $rank]);
    return $book->id;
}

function titled(string $title): string
{
    $book = Book::query()->where('title', $title)->first();
    if ($book === null) {
        return 'none';
    }
    return $book->title . '@' . $book->shelf;
}

function noteOf(int $id): ?string
{
    $book = Book::query()->find($id);
    if ($book === null) {
        return null;
    }
    return $book->note;
}

function rankOf(int $id): ?int
{
    $book = Book::find($id);
    if ($book === null) {
        return null;
    }
    return $book->rank;
}

/**
 * @return Collection<int, Book>
 */
function onShelf(int $shelf): Collection
{
    return Book::query()->where('shelf', $shelf)->get();
}

/**
 * @return list<string>
 */
function titlesOn(int $shelf): array
{
    $found = [];
    foreach (onShelf($shelf) as $book) {
        $found[] = $book->title;
    }
    return $found;
}

function countOn(int $shelf): int
{
    return onShelf($shelf)->count();
}

function anyOn(int $shelf): bool
{
    return onShelf($shelf)->isNotEmpty();
}

function firstOn(int $shelf): string
{
    $first = onShelf($shelf)->first();
    return $first === null ? '-' : $first->title;
}

function unranked(int $shelf): string
{
    $book = Book::query()->where('shelf', $shelf)->where('rank', null)->first();
    return $book === null ? 'none' : $book->title;
}

/**
 * @return list<int>
 */
function idsNoted(string $note): array
{
    $found = [];
    foreach (Book::query()->where('note', $note)->get() as $book) {
        $found[] = $book->id;
    }
    return $found;
}

function total(): int
{
    return count(Book::all());
}

function soleTitled(string $title): string
{
    try {
        $book = Book::query()->where('title', $title)->sole();
        return $book->title . '#' . $book->id;
    } catch (ModelNotFoundException $e) {
        return 'missing: ' . $e->getMessage();
    } catch (MultipleRecordsFoundException $e) {
        return 'doubled: ' . $e->getMessage();
    }
}

function firstOnOrSay(int $shelf): string
{
    try {
        return Book::query()->where('shelf', $shelf)->firstOrFail()->title;
    } catch (ModelNotFoundException $e) {
        return $e->getMessage();
    }
}

function findOrSay(int $id): string
{
    try {
        $book = Book::findOrFail($id);
        return $book->title;
    } catch (ModelNotFoundException $e) {
        return $e->getMessage();
    }
}
