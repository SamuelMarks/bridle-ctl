import { ComponentFixture, TestBed } from '@angular/core/testing';
import { WritableSignal, signal } from '@angular/core';
import { of, throwError } from 'rxjs';
import { PrSyncComponent } from './pr-sync.component';
import { OrgService } from '../../../core/services/org.service';
import { PrsStore } from '../../../core/store/prs.store';
import { NotificationService } from '../../../core/services/notification.service';
import {
  Organization,
  PullRequest,
  Repository,
} from '../../../core/models/models';

interface MockOrgService {
  orgs: WritableSignal<Organization[]>;
  repos: WritableSignal<Repository[]>;
  isLoading: WritableSignal<boolean>;
  loadOrgs: jasmine.Spy;
  ingestOrg: jasmine.Spy;
  loadRepos: jasmine.Spy;
}

interface MockPrsStore {
  prs: WritableSignal<PullRequest[]>;
  isSyncing: WritableSignal<boolean>;
  loadPrs: jasmine.Spy;
  syncPrs: jasmine.Spy;
}

interface MockNotificationService {
  success: jasmine.Spy;
  error: jasmine.Spy;
  info: jasmine.Spy;
}

describe('PrSyncComponent', () => {
  let component: PrSyncComponent;
  let fixture: ComponentFixture<PrSyncComponent>;
  let mockOrgService: MockOrgService;
  let mockPrsStore: MockPrsStore;
  let mockNotificationService: MockNotificationService;

  beforeEach(async () => {
    const initialOrgs: Organization[] = [
      { id: '1', name: 'Org 1', provider: 'github' },
    ];
    mockOrgService = {
      orgs: signal(initialOrgs),
      repos: signal([]),
      isLoading: signal(false),
      loadOrgs: jasmine.createSpy('loadOrgs').and.returnValue(of(initialOrgs)),
      ingestOrg: jasmine.createSpy('ingestOrg'),
      loadRepos: jasmine.createSpy('loadRepos'),
    };

    mockPrsStore = {
      prs: signal([
        { id: 'pr1', title: 'Test PR', repoId: 'repo1', status: 'LOCAL' },
        { id: 'pr2', title: 'Synced PR', repoId: 'repo1', status: 'SYNCED' },
        {
          id: 'pr3',
          title: 'Conflict PR',
          repoId: 'repo1',
          status: 'CONFLICT',
        },
      ]),
      isSyncing: signal(false),
      loadPrs: jasmine.createSpy('loadPrs'),
      syncPrs: jasmine.createSpy('syncPrs'),
    };

    mockNotificationService = {
      success: jasmine.createSpy('success'),
      error: jasmine.createSpy('error'),
      info: jasmine.createSpy('info'),
    };

    await TestBed.configureTestingModule({
      imports: [PrSyncComponent],
      providers: [
        { provide: OrgService, useValue: mockOrgService },
        { provide: PrsStore, useValue: mockPrsStore },
        { provide: NotificationService, useValue: mockNotificationService },
      ],
    }).compileComponents();

    fixture = TestBed.createComponent(PrSyncComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

  it('should initialize org options and auto-select first org', () => {
    const options = component.orgOptions();
    expect(options.length).toBe(1);
    expect(options[0].value).toBe('1');
    expect(mockPrsStore.loadPrs).toHaveBeenCalledWith('1');
  });

  it('should handle loadOrgs error on init', () => {
    mockOrgService.loadOrgs.and.returnValue(
      throwError(() => new Error('Failed')),
    );
    component.ngOnInit();
    expect(mockNotificationService.error).toHaveBeenCalledWith(
      'Failed to load organizations',
    );
  });

  it('should load PRs when org selection changes', () => {
    mockPrsStore.loadPrs.calls.reset();
    component.form.controls.orgId.setValue('2');
    expect(mockPrsStore.loadPrs).toHaveBeenCalledWith('2');
  });

  it('should not load PRs when org selection is empty', () => {
    mockPrsStore.loadPrs.calls.reset();
    component.form.controls.orgId.setValue('');
    expect(mockPrsStore.loadPrs).not.toHaveBeenCalled();
  });

  it('should sync PRs when form is valid', () => {
    component.form.setValue({
      orgId: '1',
      maxRate: 5,
      forkOrg: '',
    });

    component.onSync();

    expect(mockPrsStore.syncPrs).toHaveBeenCalledWith({
      orgId: '1',
      maxRate: 5,
    });
  });

  it('should not sync if form is invalid', () => {
    mockPrsStore.syncPrs.calls.reset();
    component.form.setValue({
      orgId: '',
      maxRate: 5,
      forkOrg: '',
    });
    component.onSync();
    expect(mockPrsStore.syncPrs).not.toHaveBeenCalled();
  });
});
