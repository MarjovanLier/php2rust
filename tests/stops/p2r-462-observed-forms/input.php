<?php

declare(strict_types=1);

namespace App;

use App\Observers\CodeObserver;
use App\Observers\Watcher;
use Illuminate\Database\Eloquent\Attributes\ObservedBy;
use Illuminate\Database\Eloquent\Model;

// Forms of #[ObservedBy] that P2R-460 does not admit, which keep P2R-401's stop: a string,
// a named argument, a class nobody declares, an empty list, another attribute beside it,
// an interface, and a second argument (P2R-462).
#[ObservedBy('App\Observers\CodeObserver')]
final class Str extends Model
{
}

#[ObservedBy(classes: [CodeObserver::class])]
final class Named extends Model
{
}

#[ObservedBy([Missing::class])]
final class Unknown extends Model
{
}

#[ObservedBy([])]
final class Empty1 extends Model
{
}

#[ObservedBy(CodeObserver::class)]
#[\AllowDynamicProperties]
final class Mixed1 extends Model
{
}

#[ObservedBy(Watcher::class)]
final class Iface extends Model
{
}

#[ObservedBy(CodeObserver::class, CodeObserver::class)]
final class Two extends Model
{
}

// self::class and parent::class, static::class, which PHP rejects in an attribute, a keyed
// entry, array(..), and an included observer whose parent nobody declares.
#[ObservedBy(self::class)]
final class SelfObserved extends Model
{
}

#[ObservedBy(parent::class)]
final class ParentObserved extends Model
{
}

#[ObservedBy(static::class)]
final class StaticObserved extends Model
{
}

#[ObservedBy(['k' => CodeObserver::class])]
final class Keyed extends Model
{
}

#[ObservedBy(array(CodeObserver::class))]
final class Legacy extends Model
{
}

#[ObservedBy(\App\Observers\Orphan::class)]
final class Orphaned extends Model
{
}
