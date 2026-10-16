<?php

declare(strict_types=1);

namespace App;

use Illuminate\Database\Eloquent\Model;

// $guarded = [] makes every key fillable. A nullable column without a default may be left
// out and reads null; one with a default must be named, because the created model would
// read null where the database stores the default.
final class Note extends Model
{
    protected $guarded = [];
}

// Laravel writes created_at and updated_at on create, so they need no default.
final class Stamp extends Model
{
    protected $fillable = ['label'];
}

function note(string $title, ?string $body, int $rank): Note
{
    return Note::create(['rank' => $rank, 'title' => $title, 'body' => $body, 'status' => 'new']);
}

function bare(string $title): Note
{
    return Note::query()->create(['title' => $title, 'rank' => 0, 'status' => 'draft']);
}

$first = note('one', 'text', 2);
$second = bare('two');
echo $first->id, ' ', $first->title, ' ', $first->rank, "\n";
$status = $first->status;
if ($status !== null) {
    echo $status, "\n";
}
echo $second->id, ' ', $second->title, ' ', $second->body === null ? 'no body' : 'body', "\n";
$body = $first->body;
if ($body !== null) {
    echo $body, "\n";
}
$stamp = Stamp::create(['label' => 'p']);
echo $stamp->id, ' ', $stamp->label, "\n";
