import { TestBed } from '@angular/core/testing';
import { PrsStore } from './prs.store';
import { PrService } from '../services/pr.service';
import { NotificationService } from '../services/notification.service';
import { of, throwError } from 'rxjs';
import { PullRequest } from '../models/models';

describe('PrsStore', () => {
  let store: InstanceType<typeof PrsStore>;
  let prServiceSpy: jasmine.SpyObj<PrService>;
  let notificationServiceSpy: jasmine.SpyObj<NotificationService>;

  beforeEach(() => {
    const prSpy = jasmine.createSpyObj('PrService', ['loadPrs', 'syncPrs']);
    const notifSpy = jasmine.createSpyObj('NotificationService', [
      'error',
      'success',
    ]);

    TestBed.configureTestingModule({
      providers: [
        { provide: PrService, useValue: prSpy },
        { provide: NotificationService, useValue: notifSpy },
      ],
    });

    prServiceSpy = TestBed.inject(PrService) as jasmine.SpyObj<PrService>;
    notificationServiceSpy = TestBed.inject(
      NotificationService,
    ) as jasmine.SpyObj<NotificationService>;
    store = TestBed.inject(PrsStore);
  });

  it('should be created with initial state', () => {
    expect(store.prs()).toEqual([]);
    expect(store.isSyncing()).toBeFalse();
    expect(store.isLoading()).toBeFalse();
    expect(store.error()).toBeNull();
    expect(store.activeOrgId()).toBeNull();
  });

  it('should calculate total, synced, local, and conflict PRs', () => {
    // We cannot mock internal state directly without using patchState, but patchState works on the store.
    // However, since it's a signal store, we could trigger loadPrs to set state.
    const prs: PullRequest[] = [
      { id: '1', status: 'SYNCED' } as unknown as PullRequest,
      { id: '2', status: 'LOCAL' } as unknown as PullRequest,
      { id: '3', status: 'CONFLICT' } as unknown as PullRequest,
      { id: '4', status: 'SYNCED' } as unknown as PullRequest,
    ];

    prServiceSpy.loadPrs.and.returnValue(of(prs));
    store.loadPrs('org-1');

    expect(store.totalPrs()).toBe(4);
    expect(store.syncedPrs().length).toBe(2);
    expect(store.localPrs().length).toBe(1);
    expect(store.conflictPrs().length).toBe(1);
  });

  it('should load PRs successfully', () => {
    const prs: PullRequest[] = [{ id: '1' } as unknown as PullRequest];
    prServiceSpy.loadPrs.and.returnValue(of(prs));

    store.loadPrs('org-1');

    expect(prServiceSpy.loadPrs).toHaveBeenCalledWith('org-1');
    expect(store.prs()).toEqual(prs);
    expect(store.activeOrgId()).toBe('org-1');
    expect(store.isLoading()).toBeFalse();
    expect(store.error()).toBeNull();
  });

  it('should handle error when loading PRs', () => {
    prServiceSpy.loadPrs.and.returnValue(
      throwError(() => new Error('Load failed')),
    );

    store.loadPrs('org-1');

    expect(store.error()).toBe('Load failed');
    expect(store.isLoading()).toBeFalse();
    expect(notificationServiceSpy.error).toHaveBeenCalledWith('Load failed');
  });

  it('should handle error without message when loading PRs', () => {
    prServiceSpy.loadPrs.and.returnValue(throwError(() => ({})));

    store.loadPrs('org-1');

    expect(store.error()).toBe('Failed to load PRs');
    expect(store.isLoading()).toBeFalse();
    expect(notificationServiceSpy.error).toHaveBeenCalledWith(
      'Failed to load PRs',
    );
  });

  it('should sync PRs successfully and reload if active org matches', () => {
    prServiceSpy.syncPrs.and.returnValue(of({ syncedCount: 5 }));
    // Setup initial state with an active org ID
    prServiceSpy.loadPrs.and.returnValue(of([]));
    store.loadPrs('org-1'); // Sets activeOrgId to 'org-1'
    prServiceSpy.loadPrs.calls.reset(); // clear calls

    store.syncPrs({ orgId: 'org-1', maxRate: 10 });

    expect(prServiceSpy.syncPrs).toHaveBeenCalledWith('org-1', 10);
    expect(store.isSyncing()).toBeFalse();
    expect(notificationServiceSpy.success).toHaveBeenCalledWith(
      'Successfully synced 5 PRs.',
    );
    expect(prServiceSpy.loadPrs).toHaveBeenCalledWith('org-1'); // Reloads
  });

  it('should sync PRs successfully but not reload if active org does not match', () => {
    prServiceSpy.syncPrs.and.returnValue(of({ syncedCount: 2 }));
    prServiceSpy.loadPrs.and.returnValue(of([]));
    store.loadPrs('org-2'); // Sets activeOrgId to 'org-2'
    prServiceSpy.loadPrs.calls.reset(); // clear calls

    store.syncPrs({ orgId: 'org-1', maxRate: 10 });

    expect(prServiceSpy.syncPrs).toHaveBeenCalledWith('org-1', 10);
    expect(store.isSyncing()).toBeFalse();
    expect(notificationServiceSpy.success).toHaveBeenCalledWith(
      'Successfully synced 2 PRs.',
    );
    expect(prServiceSpy.loadPrs).not.toHaveBeenCalled(); // No reload
  });

  it('should handle error when syncing PRs', () => {
    prServiceSpy.syncPrs.and.returnValue(
      throwError(() => new Error('Sync failed')),
    );

    store.syncPrs({ orgId: 'org-1', maxRate: 10 });

    expect(store.isSyncing()).toBeFalse();
    expect(notificationServiceSpy.error).toHaveBeenCalledWith('Sync failed');
  });

  it('should handle error without message when syncing PRs', () => {
    prServiceSpy.syncPrs.and.returnValue(throwError(() => ({})));

    store.syncPrs({ orgId: 'org-1', maxRate: 10 });

    expect(store.isSyncing()).toBeFalse();
    expect(notificationServiceSpy.error).toHaveBeenCalledWith(
      'Failed to sync PRs',
    );
  });
});
