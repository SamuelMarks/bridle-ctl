import { ComponentFixture, TestBed } from '@angular/core/testing';
import { WritableSignal, signal } from '@angular/core';
import { By } from '@angular/platform-browser';
import { of } from 'rxjs';
import { DashboardComponent } from './dashboard.component';
import { SystemStateService } from '../../core/services/system-state.service';
import { SystemHealth } from '../../core/models/models';

interface MockSystemStateService {
  health: WritableSignal<SystemHealth>;
  isLoading: WritableSignal<boolean>;
  checkHealth: jasmine.Spy;
}

describe('DashboardComponent', () => {
  let component: DashboardComponent;
  let fixture: ComponentFixture<DashboardComponent>;
  let mockSystemStateService: MockSystemStateService;

  beforeEach(async () => {
    mockSystemStateService = {
      health: signal({ rest: 'UP', rpc: 'DOWN', agent: 'UP' }),
      isLoading: signal(false),
      checkHealth: jasmine
        .createSpy('checkHealth')
        .and.returnValue(of({ rest: 'UP', rpc: 'DOWN', agent: 'UP' })),
    };

    await TestBed.configureTestingModule({
      imports: [DashboardComponent],
      providers: [
        { provide: SystemStateService, useValue: mockSystemStateService },
      ],
    }).compileComponents();

    fixture = TestBed.createComponent(DashboardComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });

  it('should call checkHealth on init', () => {
    expect(mockSystemStateService.checkHealth).toHaveBeenCalled();
  });

  it('should call checkHealth on refresh click', () => {
    mockSystemStateService.checkHealth.calls.reset();
    const btn = fixture.debugElement.query(By.css('app-button')).nativeElement;
    btn.click();
    expect(mockSystemStateService.checkHealth).toHaveBeenCalled();
  });

  it('should display correct health badges', () => {
    const badges = fixture.debugElement.queryAll(By.css('app-badge'));
    expect(badges.length).toBe(3);

    expect(badges[0].nativeElement.textContent.trim()).toBe('UP');
    expect(badges[0].componentInstance.variant()).toBe('success');

    expect(badges[1].nativeElement.textContent.trim()).toBe('DOWN');
    expect(badges[1].componentInstance.variant()).toBe('danger');

    expect(badges[2].nativeElement.textContent.trim()).toBe('UP');
    expect(badges[2].componentInstance.variant()).toBe('success');
  });
});
