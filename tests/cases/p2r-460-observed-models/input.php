<?php

declare(strict_types=1);

namespace App;

use App\Observers\ChildObserver;
use App\Observers\CodeObserver;
use Illuminate\Database\Eloquent\Attributes\ObservedBy;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\ModelNotFoundException;

// Models whose observer Laravel runs on create, update and delete only, so their reads
// convert; nothing creates their rows, so the store stays empty (P2R-460, P2R-461).
#[ObservedBy([CodeObserver::class])]
final class Code extends Model
{
    protected $fillable = ['name', 'rank'];

    public $timestamps = false;
}

// Two attributes in one group: a fully qualified name with ::CLASS, and a list with a
// trailing comma naming an observer with a parent and an interface.
#[ObservedBy(\App\Observers\CodeObserver::CLASS), ObservedBy([ChildObserver::class,])]
final class Tag extends Model
{
    protected $fillable = ['label'];

    public $timestamps = false;
}

function named(string $name): string
{
    $code = Code::query()->where('name', $name)->first();
    return $code === null ? '-' : $code->name;
}

function rankOf(int $id): int
{
    $code = Code::find($id);
    return $code === null ? -1 : $code->rank;
}

// A field read of a model with observers fires no event.
function label(Tag $tag): string
{
    return $tag->label;
}

function tagOf(int $id): string
{
    try {
        return label(Tag::findOrFail($id));
    } catch (ModelNotFoundException $e) {
        return $e->getMessage();
    }
}

echo named('a'), ' ', rankOf(1), ' ', tagOf(2), "\n";
