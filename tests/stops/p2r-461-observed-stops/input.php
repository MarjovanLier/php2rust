<?php

declare(strict_types=1);

namespace App;

use App\Observers\CodeObserver;
use App\Observers\HiddenObserver;
use App\Observers\InheritedObserver;
use App\Observers\OwnObserver;
use App\Observers\TraitObserver;
use App\Observers\UpperObserver;
use Illuminate\Database\Eloquent\Attributes\ObservedBy;
use Illuminate\Database\Eloquent\Model;

// create on a model with observers, which Laravel runs and the store does not (P2R-461).
#[ObservedBy([CodeObserver::class])]
final class Code extends Model
{
    protected $fillable = ['name'];

    public $timestamps = false;
}

function made(): Code
{
    return Code::create(['name' => 'a']);
}

// Reads of models whose observer has retrieved: its own, in another letter case,
// inherited, from a trait and private, as method_exists finds each (P2R-461).
#[ObservedBy([OwnObserver::class])]
final class Own extends Model
{
    public $timestamps = false;
}

#[ObservedBy([UpperObserver::class])]
final class Upper extends Model
{
    public $timestamps = false;
}

#[ObservedBy([CodeObserver::class, InheritedObserver::class])]
final class Inherited extends Model
{
    public $timestamps = false;
}

#[ObservedBy(TraitObserver::class)]
final class Traited extends Model
{
    public $timestamps = false;
}

#[ObservedBy([HiddenObserver::class])]
final class Hidden extends Model
{
    public $timestamps = false;
}

// Another model as the observer, which takes retrieved from Model (HasEvents).
#[ObservedBy([Own::class])]
final class Peer extends Model
{
    public $timestamps = false;
}

function reads(): bool
{
    return Peer::find(1) === null
        && Own::find(1) === null
        && Upper::query()->where('name', 'a')->first() === null
        && Inherited::find(1) === null
        && Traited::find(1) === null
        && Hidden::find(1) === null;
}
